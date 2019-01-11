use std::thread;
use crossbeam_channel::bounded;

mod doge;
// use threadpool::ThreadPool;
// use spmc;
fn work(s: crossbeam_channel::Sender<Option<String>> , r: crossbeam_channel::Receiver<Option<String>> , n_workers: &u32, counter: &mut doge::WordGenerator){
    for n in 0..*n_workers{
        let rx = r.clone();
        thread::spawn(move || {
            let mut _c = 0;
            loop{
                match rx.recv().unwrap(){
                    Some(i) =>{
                        _c += 1;
                        continue;
                    },
                    None => {
                        println!("Worker {} recieved {}", n, _c);
                        break;
                    },
                } 
            
            }
        });
    }

    for i in counter{
        s.send(Some(i.clone())).unwrap_or_else(|err|{
            println!("Channel has died");
        });
    }
    for i in 0..*n_workers{
        s.send(None).unwrap_or_else(|err|{
            println!("Failed to send terminating signal to threads: {}", err);
        });
    }
}

fn main() {
    let user_creds = doge::UserCreds::parse_user_input();


    let vocab = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let mut cnt = 0;
    // let n_workers = 7;
    let mut counter = doge::WordGenerator::new(6, vocab);
    let (s, r) = bounded(user_creds.n_workers as usize);
    work(s, r, &user_creds.n_workers, &mut counter);

    println!("Total count: {}", cnt);
}
