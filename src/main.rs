use std::mem;

struct Counter {
    count: usize,
    cmax: usize
}

impl Counter {
    fn new(cmax : usize) -> Counter {
        Counter { count: 0, cmax: cmax }
    }
}

struct WordGenerator {
    stringLen: usize,
    counts: Vec<usize>,
    vocab: Vec<char>,
    isStart: bool
}

impl WordGenerator {
    fn convertWord(&self) -> String {
        self.counts.iter().map(|x| self.vocab[*x]).collect()
    }

    fn new(targetLen : usize, vocab : Vec<char>) -> WordGenerator {

        WordGenerator {
            stringLen: targetLen,
            counts: vec![0; targetLen],
            vocab: vocab,
            isStart: true
        }
    }
}

impl Iterator for WordGenerator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if self.isStart {
            self.isStart = false;
            return Some(self.convertWord());
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
            Some(self.convertWord())
        }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.count += 1;

        if self.count < self.cmax {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let vocab = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let mut cnt = 0;

    let counter = WordGenerator::new(6, vocab);
    for i in counter {
        cnt += 1;
    }

    println!("Total count: {}", cnt);
}
