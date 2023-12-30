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
    // TODO:
    // REMEMBER TO USE THE CHECKED ANNOTATION FOR <CORRECT> IN ORDER TO SEE IF PERFORMANCE WILL IMPROVE
    pub fn possible_matches(&self, word: &str) -> bool {
        let mut marked = [false; 5]; // used to annotate previous guess word
        let mut checked = [false; 5]; // used to annotate current_word in CORRECTNESS::MISPLACED

        'outer: for (i, ((guess_char, m), current_word_char)) in self
            .word
            .chars()
            .zip(self.mask.iter())
            .zip(word.chars())
            .enumerate()
        {
            match *m {
                // check that all CORRECT characters are present with right position,
                Correctness::Correct => {
                    if guess_char != current_word_char {
                        return false;
                    } else {
                        marked[i] = true;
                        // TODO:
                        // uncomment the line below to see if it would increase performance
                        checked[i] = true;
                        continue;
                    }
                }

                // check that no WRONG characters are present in the current word
                Correctness::Wrong => {
                    if word.chars().enumerate().any(|(i, current_word_char)| {
                        guess_char == current_word_char && !checked[i]
                    }) {
                        return false;
                    } else {
                        marked[i] = true;
                        continue;
                    }
                }
                Correctness::Misplaced => {
                    for (i, w) in word.chars().enumerate() {
                        // check that all the MISPLACED character are in the guess word
                        if !checked[i] && guess_char == w {
                            checked[i] = true;
                            continue 'outer;
                        }
                    }
                    return false;
                }
            }
        }
        // word passes all the tests so it must match
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
        let mut checked = [false; 5];

        // Green
        for (i, (a, g)) in answer.chars().zip(guess.chars()).enumerate() {
            if a == g {
                c[i] = Correctness::Correct;
                marked[i] = true;
                checked[i] = true;
            }
        }

        // Misplaced
        for (i, g) in guess.chars().enumerate() {
            if !checked[i] {
                checked[i] = true;
                if answer.chars().enumerate().any(|(k, a)| {
                    if !marked[k] && a == g {
                        marked[k] = true;
                        return true;
                    } else {
                        false
                    }
                }) {
                    c[i] = Correctness::Misplaced;
                }
            }
            //if answer.chars().any(|a| {
            //    if a == g && !marked[i] {
            //        marked[i] = true;
            //        return true;
            //    } else {
            //        // already marked
            //        false
            //    }
            //}) {
            //    c[i] = Correctness::Misplaced;
            //    //marked[i] = true;
            //}
        }
        c
    }

    pub fn compose() -> Vec<[Correctness; 5]> {
        //assert_eq!(a.len(), b.len());
        let a = [
            Correctness::Correct,
            Correctness::Misplaced,
            Correctness::Wrong,
        ];
        let b = a.clone();
        let mut res1 = Vec::new();

        for i in a {
            for j in &b {
                res1.push([i, *j]);
            }
        }

        // second compose
        let mut res2 = Vec::new();
        for i in &res1 {
            for k in &res1 {
                res2.push([i[0], i[1], k[0], k[1]]);
            }
        }

        // final compose
        let mut result = Vec::new();
        for i in res2 {
            for k in &b {
                result.push([i[0], i[1], i[2], i[3], *k]);
            }
        }
        result
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
    ($prev:literal + [$($mask:tt)+] allows $next: literal) => {
        $crate::Guess { word: $prev.to_string(), mask: coret![$($mask)+], }.matches($next)
    };
}
