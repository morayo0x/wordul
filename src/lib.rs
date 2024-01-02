use std::{borrow::Cow, collections::HashSet};
pub mod algorithm;
//use std::io::Write;
const DICTIONARY: &'static str = include_str!("../dictionary.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Correctness {
    Correct,
    Misplaced,
    Wrong,
}

pub struct Guess<'a> {
    pub word: Cow<'a, str>,
    pub mask: [Correctness; 5],
}

impl Guess<'_> {
    // the function of matches is to check whether the potential next guess = word
    // ... by making sure that it contains all the Corect, Misplaced, and NO Wrong character
    // ... using the coretness of the previous guess = self.word
    pub fn matches(&self, word: &str) -> bool {
        // novel insight.- because I have had to write a new algorithm to approach the problem
        // ... turns out I could re-use the implementation of Correctness::compute
        //
        // We assume guess = self.word && answer = word.
        //
        // Based on this assumption,
        // The code below will check;
        //  ... that word contains all the Correct chars in the right posiiton
        // ... it will check that all the Misplaced character are
        //  ...  (i). are present
        //   ... (ii)  are not in the same position as the last guess, otherwise they would be Correct
        //   .. no Wrong characters of guess are in word
        return Correctness::compute(word, &self.word) == self.mask;
    }
}

pub trait Guesser {
    fn guess(&mut self, history: &[Guess<'_>]) -> String;
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
        let mut history = Vec::new();
        for i in 1..=6 {
            let guess = guesser.guess(&history);
            assert!(self.dictionary.contains(&*guess));
            if answer == &guess {
                return Some(i);
            }

            let coretness = Correctness::compute(answer, &guess);
            history.push(Guess {
                word: Cow::Owned(guess),
                mask: coretness,
            });
            //
            //// TO TEST AGAINST A REAL WORLD WORDUL, WE UNCOMMENT THE LINES BELOW
            // println!("Guess is {}", guess);
            // println!();
            // print!("Enter the correctness of the character in lower case: ");
            // //sometimes the stdout maybe buffered, especially when using print!()
            //... and so we call flush to print the content immediately
            // io::stdout().flush().unwrap();
            // let mut buf = String::new();
            // io::stdin().read_line(&mut buf).unwrap();
            //
            // let mut coretness = Vec::new();
            //     buf.trim().to_lowercase().chars().for_each(|c| match c {
            //         'c' => coretness.push(Correctness::Correct),
            //         'm' => coretness.push(Correctness::Misplaced),
            //         'w' => coretness.push(Correctness::Wrong),
            //         _ => panic!("a wrong character"),
            //     });
            //
            //     history.push(Guess {
            //         word: Cow::Owned(guess),
            //         mask: coretness.try_into().unwrap_or_else(|c: Vec<_>| {
            //             panic!("Expected an array of length of 5 got {}", c.len())
            //         }),
            //     });
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
        let mut marked = [false; 5]; // used to annotate answer
        let mut checked = [false; 5]; // used to annotate guess

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
        }
        c
    }

    pub fn compose() -> Vec<[Correctness; 5]> {
        //assert_eq!(a.len(), b.len());
        let a = [
            Correctness::Wrong,
            Correctness::Correct,
            Correctness::Misplaced,
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

pub enum CowUser<'a, B>
where
    B: 'a + ToOwned + ?Sized,
{
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}
