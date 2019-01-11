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
    let vocab = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let mut cnt = 0;
    let target = hex::decode("5681c3763ecd8a27cbead0b79fef20c8").expect("Ooops");

    // let output = [0; 16];
    // let mut hasher = Md5::new();
    let counter = WordGenerator::new(4, vocab);

    for x in counter {
        
        // hasher.input(x.as_bytes());

        // hasher.result(&mut output);
        // println!("Trying {}", x);

        if create_hash(x.as_bytes(), 1000) == target[..] {
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



