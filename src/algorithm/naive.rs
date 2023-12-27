#[allow(unused_imports)]
use crate::{Guess, Guesser, DICTIONARY};
use std::collections::HashMap;

pub struct Naive {
    pub remaining: HashMap<&'static str, usize>,
}

impl Naive {
    pub fn new() -> Self {
        Naive {
            remaining: HashMap::from_iter(DICTIONARY.lines().map(|line| {
                let (word, count) = line
                    .split_once(" ")
                    .expect("Everyl line is Word + Space + Count");

                let count = count.parse().expect("Count is not a number");

                (word, count)
            })),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Candidate {
    word: &'static str,
    count: usize,
    goodness: f64,
}

//impl Guesser for Naive {
//    fn guess(&mut self, history: &[Guess]) -> String {
//        if Some(last) = history.last() {
//            self.remaining.retain(|word, _| last.matches(word))
//        }

//        // when there are no previous guesses, then guess the best one
//        let mut best: Option<Candidate> = None;
//        for (&word, &count) in &self.remaining {
//            // TODO: How to compute goodness
//            let goodness = 0.0;
//            if let Some(c) = best {
//                if goodness > c.goodness {
//                    best = Some(Candidate {
//                        word,
//                        count,
//                        goodness,
//                    })
//                }
//            } else {
//                best = Some(Candidate {
//                    word,
//                    count,
//                    goodness,
//                })
//            }
//        }
//        best.unwrap().word.to_string()
//    }
//}
