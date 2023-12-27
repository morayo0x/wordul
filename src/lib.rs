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

impl Guess {
    pub fn matches(&self, word: &str) -> bool {
        let mut marked = [false; 5]; // used to annotate previous guess word
        let mut used = [false; 5]; // used to annotate current word

        for (i, ((guess_char, m), current_word_char)) in self
            .word
            .chars()
            .zip(self.mask.iter())
            .zip(word.chars())
            .enumerate()
        {
            match *m {
                // Check that all the CORRECT characters are present,
                //.. and have the right positional index in the current word
                Correctness::Correct => {
                    if guess_char != current_word_char {
                        //res = false;
                        return false;
                    } else {
                        marked[i] = true;
                        used[i] = true;
                        continue;
                    }
                }

                // check that no WRONG characters are present in the current word
                Correctness::Wrong => {
                    if word.chars().enumerate().any(|(k, current_word_char)| {
                        // checked that the current_word_char has not been used
                        if !used[k] && guess_char == current_word_char {
                            return false;
                        } else {
                            used[k] = true;
                            marked[i] = true;
                            true
                        }
                    }) {}
                }

                _ => continue,
            }
        }

        // check that all the MISPLACED character are present even when repeated
        // NOTE: there is no use for checking marked[i] since we are certain that
        // ... only MISPLACED character are left as CORRECT and WRONG ones have been checked
        for (guess_char, m) in self.word.chars().zip(self.mask.iter()) {
            if *m == Correctness::Misplaced {
                // find a misplaced character that does not belong to word
                if word.chars().enumerate().any(|(k, current_word_char)| {
                    // check that the current_word_char has not been used for CORRECT
                    if !used[k] && guess_char == current_word_char {
                        used[k] = true;
                        true
                    } else {
                        return false;
                    }
                }) {
                    continue;
                }
            }
        }
        true
    }
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

#[macro_export]
macro_rules! check_matches {
    ($prev:literal + [$($mask:tt)+] allows $next:literal) => {
        assert!(
            $crate::Guess {
                word: $prev.to_string(),
                mask: coret![$($mask)+],
            }
            .matches($next)
    )
    };

     ($prev:literal + [$($mask:tt)+] disallows $next:literal) => {
        assert!(
            !$crate::Guess {
                word: $prev.to_string(),
                mask: mask![$($mask)+],
            }
            .matches($next)
        )
    };
}
