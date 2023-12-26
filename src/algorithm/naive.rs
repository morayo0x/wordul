use std::collections::HashMap;

use crate::{Guess, Guesser, DICTIONARY};

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

impl Guesser for Naive {
    fn guess(&mut self, history: &[Guess]) -> String {
        "hello".to_string()
    }
}
