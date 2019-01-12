extern crate crypto;

use hex;
use crypto::md5::Md5;
use crypto::digest::Digest;

struct WordGenerator {
    counts: Vec<usize>,
    vocab: Vec<char>,
    is_start: bool
}

impl WordGenerator {
    fn convert_word(&self) -> String {
        self.counts.iter().map(|x| self.vocab[*x]).collect()
    }

    fn new(target_len : usize, vocab : Vec<char>) -> WordGenerator {
        WordGenerator {
            counts: vec![0; target_len],
            vocab: vocab,
            is_start: true
        }
    }
}

impl Iterator for WordGenerator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if self.is_start {
            self.is_start = false;
            return Some(self.convert_word());
        }

        let len = self.counts.len();
        let mut all_full = false;

        for i in 1..len+1 {
            if self.counts[len - i] < self.vocab.len() - 1 {
                self.counts[len - i] += 1;
                break;
            } else if i == len {
                all_full = true;
            } else {
                for j in 1..i + 1 {
                    self.counts[len - j] = 0;
                }
            }
        }

        if all_full {
            None
        } else {
            Some(self.convert_word())
        }
    }
}

struct Md5Crypt {
    password: Vec<u8>,
    salt: Vec<u8>,
    magic: Vec<u8>,
    hasher: Md5
}

impl Md5Crypt {
    fn new (password: &String, salt: &String, magic: &String) -> Md5Crypt {
        Md5Crypt {
            password: password.as_bytes().to_vec(),
            salt: salt.as_bytes().to_vec(),
            magic: "$1$".as_bytes().to_vec(),
            hasher: Md5::new()
        }
    }

    fn alternate_sum (&self) -> [u8; 16] {
        let mut all = self.password.clone();
        all.extend(&self.salt);
        all.extend(&self.password);
        
        create_hash(&all[..], 1)
    }

    fn intermediate_sum (&self) -> [u8; 16] {
        let mut output: [u8; 16] = [0; 16];
        let mut hasher = Md5::new();

        // Steps 3.1 - 3.3
        hasher.input(&self.password);
        hasher.input(&self.magic);
        hasher.input(&self.salt);

        // We don't need to handle the case where password.length()
        //  exceeds the 16 bytes because our passwords won't exceed
        //  16 bytes.
        let pass_len = self.password.len();
        assert!(pass_len <= 16); // We assert this just in case

        // Step 3.4: Append pass_len bytes of the alternate sum
        hasher.input(&self.alternate_sum());

        let null_byte: [u8; 1] = [0; 1];
        let mut tmp: u8 = pass_len as u8;

        // Step 3.5: iterate over every bit in the length of the password
        while tmp != 0 {
            if tmp & 1 == 1 {
                hasher.input(&null_byte);
            } else {
                hasher.input(&self.password[0..1]);
            }
            tmp >>= 1;
        }
        hasher.result(&mut output);

        return output;
    }

    fn hash (&self) -> [u8; 16] {
        let i0 = self.intermediate_sum();
        let mut last_i = i0;
        let mut hasher = Md5::new();

        for i in 1..1000 {
            if i % 2 == 0 {
                hasher.input(&last_i);
            } else {
                hasher.input(&self.password);
            }
            if i % 3 != 0 {
                hasher.input(&self.salt);
            }
            if i % 7 != 0 {
                hasher.input(&self.password);
            }
            if i % 2 == 0 {
                hasher.input(&self.password);
            } else {
                hasher.input(&last_i);
            }
            hasher.result(&mut last_i);
            hasher.reset();
        }

        return last_i;
    }
}

fn create_hash (data: &[u8], iterations: usize) -> [u8; 16] {
    let mut output : [u8; 16] = [0; 16];
    let mut hasher = Md5::new();

    for i in 0..iterations {
        if i == 0 {
            hasher.input(data);
        } else {
            // println!("Using {} as input", hex::encode(&output));
            hasher.input(&output);
        }
        hasher.result(&mut output);
        // println!("Result: {} at iteration #{}", hex::encode(&output), &i);
        hasher.reset();
    }
    return output;
}

fn main() {
    let vocab = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let mut cnt = 0;
    let target = hex::decode("5681c3763ecd8a27cbead0b79fef20c8").expect("Ooops");

    // let output = [0; 16];
    // let mut hasher = Md5::new();
    let counter = WordGenerator::new(4, vocab);

    for x in counter {
        
        // hasher.input(x.as_bytes());

        // hasher.result(&mut output);
        // println!("Trying {}", x);

        if create_hash(x.as_bytes(), 1000) == &target[..] {
            println!("Done w/ {}", x);
            break;
        }

        // if format!("{:x}", md5::compute(&x)) == target {
        //     println!("Done w/ {}", x);
        // }
        // hasher.reset();  
        cnt += 1;
    }

    println!("Total count: {}", cnt);
    // println!("Last: {}", last);
}
