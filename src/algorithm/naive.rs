use crate::{Correctness, Word};
#[allow(unused_imports)]
use crate::{Guess, Guesser, DICTIONARY};

pub struct Naive {
    pub remaining: Vec<(Word, usize)>,
}

impl Naive {
    pub fn new() -> Self {
        Naive {
            remaining: Vec::from_iter(DICTIONARY.lines().map(|line| {
                let (word, count) = line
                    .split_once(" ")
                    .expect("Everyl line is Word + Space + Count");

                let count = count.parse().expect("Count is not a number");
                let word = word.as_bytes().try_into().unwrap();

                (word, count)
            })),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Candidate {
    pub word: Word,
    pub goodness: f64,
}

impl Guesser for Naive {
    fn guess(&mut self, history: &[Guess]) -> Word {
        // compute the next POSSIBLE words based on the correctness of the last Guess
        if let Some(last) = history.last() {
            self.remaining.retain(|&(w, _)| last.matches(w));
        }

        if history.is_empty() {
            return "tares".as_bytes().try_into().unwrap();
        }
        //*(&self.remaining.iter().count()) as f64;
        // compute the Shannon Measure of Information(SMI) for the remaining POSSIBLE words
        // .. we return the word with the highest SMI as the next guess

        // SMI = -SUM over all possible patterns of [(p_word(that match a give patter) * log(p_word))]
        let total_remaining = self.remaining.iter().map(|(_, count)| count).sum::<usize>() as f64;
        //

        //let total_remaining = self.remaining.iter().count() as f64;
        let mut best: Option<Candidate> = None;
        for &(word, count) in &self.remaining {
            let mut goodness = 0.0;

            for pattern in Correctness::compose() {
                let mut total: usize = 0;

                // TODO: could self.remaining be the Dictionary word?
                for &(candidate, count) in &self.remaining {
                    if (Guess {
                        word,
                        mask: pattern,
                    }
                    .matches(candidate))
                    {
                        total += count;
                    }
                }
                let p_word = total as f64 / total_remaining as f64;
                if p_word == 0.0 {
                    continue;
                } else {
                    goodness += -p_word * p_word.log2();
                }
            }
            // Applying weight to the goodness based on the likelihood of each word;
            let p_word = count as f64 / total_remaining;
            let goodness = p_word * goodness;
            match best {
                Some(c) => {
                    if goodness > c.goodness {
                        best = Some(Candidate { word, goodness });
                    }
                }
                None => {
                    best = Some(Candidate { word, goodness });
                }
            }
        }

        best.unwrap().word
    }
}
