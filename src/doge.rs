use smallvec::SmallVec;
#[allow(unused_imports)]
use std::env;
use std::process;

pub struct WordGenerator {
    counts: SmallVec<[u8; 10]>,
    start_byte: u8,
    end_byte: u8,
    is_start: bool,
}

impl WordGenerator {
    pub fn new(target_len: usize) -> WordGenerator {
        WordGenerator {
            counts: SmallVec::<[u8; 10]>::from_elem(97, target_len),
            start_byte: 97,
            end_byte: 122,
            is_start: true,
        }
    }

    pub fn get_size(&self) -> u64 {
        let vsize: u64 = u64::from(if self.end_byte > self.start_byte {
                self.end_byte - self.start_byte
            } else {
                self.start_byte - self.end_byte
            });
        let csize: u32 = self.counts.len() as u32;
        vsize.pow(csize)
    }
}

impl Iterator for WordGenerator {
    type Item = SmallVec<[u8; 10]>;

    fn next(&mut self) -> Option<SmallVec<[u8; 10]>> {
        if self.is_start {
            self.is_start = false;
            return Some(self.counts.clone());
        }

        let len = self.counts.len();
        let mut all_full = false;

        for i in 1..=len {
            if self.counts[len - i] < self.end_byte {
                self.counts[len - i] += 1;
                break;
            } else if i == len {
                all_full = true;
            } else {
                for j in 1..=i {
                    self.counts[len - j] = self.start_byte;
                }
            }
        }

        if all_full {
            None
        } else {
            Some(self.counts.clone())
        }
    }
}
pub struct UserCreds {
    pub password: String,
    pub salt: String,
    pub n_workers: u32,
}

impl UserCreds {
    fn new(args: &[String]) -> UserCreds {
        if args.len() < 4 {
            println!("Not enough arguments");
            process::exit(1);
        }

        let password = args[1].clone();
        let salt = args[2].clone();
        let n_workers: u32 = match args[3].trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please provide an integer");
                process::exit(1);
            }
        };
        UserCreds {
            password,
            salt,
            n_workers,
        }
    }
    pub fn parse_user_input() -> UserCreds {
        let user_args: Vec<String> = env::args().collect();
        UserCreds::new(&user_args)
    }
}
