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
use smallvec::SmallVec;
use std::thread;
use std::process;
use std::time::Instant;

const B64_ALPH: [char; 64] = ['.','/','0','1','2','3','4','5','6','7','8','9','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z','a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];

fn work(
    s: &Sender<Option<SmallVec<[u8; 10]>>>,
    r: &Receiver<Option<SmallVec<[u8; 10]>>>,
    n_workers: u32,
    counter: &mut WordGenerator,
    pass_bytes: &'static str,
    salt_bytes: &'static [u8]
) {
    let pbar = ProgressBar::new(counter.get_size());
    pbar.set_style(
        ProgressStyle::default_bar().template(
            "[{elapsed_precise}] {bar:50.cyan/blue} {pos:>7}/{len:7} {eta_precise} {msg}",
        ),
    );
    let now = Instant::now();

    for n in 0..n_workers {
        let rx = r.clone();
        thread::spawn(move || {
            // Slowish O(26 * n) op, but we're only doing it once per thread anyways
            let pass: Vec<u8> = pass_bytes.chars()
                .map(|x| B64_ALPH.iter().position(|&y| y == x).unwrap() as u8)
                .collect();
            let mut _c = 0;

            loop {
                match rx.recv().unwrap() {
                    Some(i) => {
                        let hasher = Md5Crypt::new(&i, &salt_bytes);
                        let res = hasher.hash();
                        if pass == res {
                            let out = std::str::from_utf8(&i).unwrap();
                            println!("Found password!: {}", &out);
                            process::exit(0);
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
    let total_size = counter.get_size();
    for i in counter {
        s.send(Some(i)).unwrap_or_else(|_err| {
            println!("Channel has died");
        });
        sent_cnt += 1;
        if sent_cnt % 25000 == 0 {
            pbar.inc(25000);
        }
    }
    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Done producing combinations!");
    println!("Seconds Elapsed: {}", sec);
    println!("Hashes per second: {}", total_size as f64 / sec);

    for _i in 0..n_workers {
        s.send(None).unwrap_or_else(|err| {
            println!("Failed to send terminating signal to threads: {}", err);
        });
    }
    // pbar.finish();
}

fn main() {
    let user_creds = UserCreds::parse_user_input();

    let mut counter = WordGenerator::new(6);
    let (s, r) = bounded(user_creds.n_workers as usize);
    
    let salt_bytes = b"hfT7jp2q";
    let pass = "/sDfNdP2e3OCxg2zGq1FK0";

    work(&s, &r, user_creds.n_workers, &mut counter, &pass, salt_bytes);
}
