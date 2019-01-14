extern crate base64;
extern crate crypto;

mod doge;
mod md5_crypt;

use crate::doge::{UserCreds, WordGenerator};
use crate::md5_crypt::Md5Crypt;

use crossbeam_channel::{bounded, Receiver, Sender};
use hex;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use std::thread;

fn work(
    s: &Sender<Option<String>>,
    r: &Receiver<Option<String>>,
    n_workers: u32,
    counter: &mut WordGenerator,
) {
    let pbar = ProgressBar::new(counter.get_size());
    pbar.set_style(
        ProgressStyle::default_bar().template(
            "[{elapsed_precise}] {bar:50.cyan/blue} {pos:>7}/{len:7} {eta_precise} {msg}",
        ),
    );

    for n in 0..n_workers {
        let rx = r.clone();
        thread::spawn(move || {
            let mut _c = 0;
            loop {
                match rx.recv().unwrap() {
                    Some(i) => {
                        let hasher = Md5Crypt::new(&i, &String::from("abcabc"));
                        let res = hasher.hash();
                        if &res == b"abcdabcdabcdabcd" {
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

    let mut sent_cnt: u32 = 0;
    for i in counter {
        s.send(Some(i)).unwrap_or_else(|_err| {
            println!("Channel has died");
        });
        sent_cnt += 1;
        if sent_cnt % 25000 == 0 {
            pbar.inc(25000);
        }
    }
    for _i in 0..n_workers {
        s.send(None).unwrap_or_else(|err| {
            println!("Failed to send terminating signal to threads: {}", err);
        });
    }
    pbar.finish();
}

fn main() {
    let vocab = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let user_creds = UserCreds::parse_user_input();

    // let n_workers = 7;
    let mut counter = WordGenerator::new(6, vocab);
    let (s, r) = bounded(user_creds.n_workers as usize * 10000);
    work(&s, &r, user_creds.n_workers, &mut counter);

    // let gen = WordGenerator::new();
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
