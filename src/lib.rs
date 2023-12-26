use std::collections::HashSet;
pub mod algorithm;
const DICTIONARY: &'static str = include_str!("../dictionary.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Correctness {
    Correct,
    Misplaced,
    Wrong,
}

pub struct Guess {
    pub word: String,
    pub mask: [Correctness; 5],
}

pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}

pub struct Wordle {
    pub dictionary: HashSet<&'static str>,
}

impl Wordle {
    pub fn new() -> Self {
        Self {
            dictionary: HashSet::from_iter(DICTIONARY.lines().map(|line| {
                line.split_once(" ")
                    .expect("Everry line is: Word + Space + Count")
                    .0
            })),
        }
    }

    pub fn play<G: Guesser>(&self, answer: &str, mut guesser: G) -> Option<usize> {
        // play the game a certain number of times
        let mut history = Vec::new();
        for i in 1..32 {
            let guess = guesser.guess(&history);

            assert!(self.dictionary.contains(&*guess));
            if answer == &guess {
                return Some(i);
            }
            let coretness = Correctness::compute(answer, &guess);
            history.push(Guess {
                word: guess,
                mask: coretness,
            });
        }
        None
    }
}

impl Correctness {
    pub fn compute(answer: &str, guess: &str) -> [Correctness; 5] {
        // check the lenght of answer ang guess
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);

        //
        let mut c = [Correctness::Wrong; 5];
        let mut marked = [false; 5];

        // Green
        for (i, (a, g)) in answer.chars().zip(guess.chars()).enumerate() {
            if a == g {
                c[i] = Correctness::Correct;
                marked[i] = true;
            }
        }

        // Misplaced
        for (i, a) in answer.chars().enumerate() {
            if guess.chars().any(|g| {
                if a == g && !marked[i] {
                    marked[i] = true;
                    return true;
                } else {
                    // already marked
                    false
                }
            }) {
                c[i] = Correctness::Misplaced;
                marked[i] = true;
            }
        }
        c
    }
}

#[macro_export]
macro_rules! coret {
    (C) => {
        Correctness::Correct
    };
    (M) => {
        Correctness::Misplaced
    };
    (W) => {
        Correctness::Wrong
    };


    ($($c:tt)+) => {[
        $(coret!($c)), +
    ]};
}
