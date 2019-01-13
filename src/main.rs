extern crate base64;
extern crate crypto;

use crossbeam_channel::bounded;
use crypto::digest::Digest;
use crypto::md5::Md5;
use hex;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use std::thread;

mod doge;
// use threadpool::ThreadPool;
// use spmc;

const BYTE_ORDERINGS: [usize; 16] = [11, 4, 10, 5, 3, 9, 15, 2, 8, 14, 1, 7, 13, 0, 6, 12];

fn work(
    s: crossbeam_channel::Sender<Option<String>>,
    r: crossbeam_channel::Receiver<Option<String>>,
    n_workers: &u32,
    counter: &mut doge::WordGenerator,
) {
    let bar = ProgressBar::new(counter.get_size());
    bar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:50.cyan/blue} {pos:>7}/{len:7} {eta_precise} {msg}"));

    for n in 0..*n_workers {
        let rx = r.clone();
        thread::spawn(move || {
            let mut _c = 0;
            loop {
                match rx.recv().unwrap() {
                    Some(i) => {
                        let hasher = Md5Crypt::new(&i, &String::from("abcabc"));
                        let res = hasher.hash();
                        if res == "abcdabcdabcdabcd".as_bytes() {
                            break;
                        }
                        continue;
                    }
                    None => {
                        println!("Worker {} recieved {}", n, _c);
                        break;
                    }
                }
            }
        });
    }

    let mut sent_cnt : u32 = 0;
    for i in counter {
        s.send(Some(i)).unwrap_or_else(|_err| {
            println!("Channel has died");
        });
        sent_cnt += 1;
        if sent_cnt % 25000 == 0 {
            bar.inc(25000);
        }
    }
    for _i in 0..*n_workers {
        s.send(None).unwrap_or_else(|err| {
            println!("Failed to send terminating signal to threads: {}", err);
        });
    }
    bar.finish();
}

struct Md5Crypt {
    password: Vec<u8>,
    salt: Vec<u8>,
    magic: Vec<u8>,
}

impl Md5Crypt {
    fn new(password: &String, salt: &String) -> Md5Crypt {
        Md5Crypt {
            password: password.as_bytes().to_vec(),
            salt: salt.as_bytes().to_vec(),
            magic: "$1$".as_bytes().to_vec(),
        }
    }

    fn alternate_sum(&self) -> [u8; 16] {
        let mut all = self.password.clone();
        all.extend(&self.salt);
        all.extend(&self.password);

        create_hash(&all[..], 1)
    }

    fn intermediate_sum(&self) -> [u8; 16] {
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

        let alt_sum = self.alternate_sum();
        // Step 3.4: Append pass_len bytes of the alternate sum
        hasher.input(&alt_sum[..pass_len]);
        // println!("Alternate sum: {}", hex::encode(&alt_sum));

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

    fn hash(&self) -> [u8; 16] {
        let i0 = self.intermediate_sum();
        // println!("Intermediate sum: {}", hex::encode(&i0));
        let mut last_i = i0;
        let mut hasher = Md5::new();
        // let mut debug_vec: Vec<u8> = Vec::new();

        for i in 0..1000 {
            if i & 1 == 1 {
                hasher.input(&self.password);
                // debug_vec.extend(&self.password);
            } else {
                hasher.input(&last_i);
                // debug_vec.extend(&last_i);
            }
            if i % 3 > 0 {
                hasher.input(&self.salt);
                // debug_vec.extend(&self.salt);
            }
            if i % 7 > 0 {
                hasher.input(&self.password);
                // debug_vec.extend(&self.password);
            }
            if i & 1 == 1 {
                hasher.input(&last_i);
                // debug_vec.extend(&last_i);
            } else {
                hasher.input(&self.password);
                // debug_vec.extend(&self.password);
            }
            hasher.result(&mut last_i);
            hasher.reset();
            if i < 10 || i > 990 {
                // println!("{}: Input: {}", i, hex::encode(&debug_vec));
                // println!("{}: {} of len {}", i, hex::encode(&last_i), debug_vec.len());
            }
            // debug_vec.clear();
        }

        return Md5Crypt::reorder_bytes(&last_i);
    }

    fn reorder_bytes(original: &[u8]) -> [u8; 16] {
        let mut output: [u8; 16] = [0; 16];
        for i in 0..BYTE_ORDERINGS.len() {
            output[BYTE_ORDERINGS.len() - i - 1] = original[BYTE_ORDERINGS[i]];
        }
        return output;
    }
}

fn create_hash(data: &[u8], iterations: usize) -> [u8; 16] {
    let mut output: [u8; 16] = [0; 16];
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
    let vocab = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let user_creds = doge::UserCreds::parse_user_input();

    // let n_workers = 7;
    let mut counter = doge::WordGenerator::new(6, vocab);
    let (s, r) = bounded(user_creds.n_workers as usize * 10000);
    work(s, r, &user_creds.n_workers, &mut counter);

    // let gen = doge::WordGenerator::new();
    // let pass = String::from("a");
    // let salt = String::from("bc");

    // let crypt = Md5Crypt::new(&pass, &salt);
    // let res = crypt.hash();
    // let output = base64::encode_config(&res, base64::CRYPT);
    // println!("Output: {}", output);
    // println!("Output (hex): {}", hex::encode(&res));

    // let input = base64::decode_config("HgqpXhm.E0eACNRZkZJa", base64::CRYPT);

    // match input {
    //     Ok(input) => println!("Target: {}", hex::encode(input)),
    //     Err(err) => {
    //         panic!("There was an error: {:?}", err)
    //     },
    // };

    // println!("Total count: {}", cnt);
    // println!("Last: {}", last);
}
