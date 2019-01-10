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

fn main() {
    let vocab = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let mut cnt = 0;

    let counter = WordGenerator::new(6, vocab);
    for _i in counter {
        cnt += 1;
    }

    println!("Total count: {}", cnt);
}
