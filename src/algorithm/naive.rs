use crate::{Correctness, Word};
use crate::{Guess, Guesser, DICTIONARY};
use std::{borrow::Cow, sync::OnceLock};

static INITIAL: OnceLock<Vec<(Word, usize)>> = OnceLock::new();
static PATTERN: OnceLock<Vec<[Correctness; 5]>> = OnceLock::new();

#[derive(Debug, Clone, Copy)]
pub struct Candidate {
    pub word: Word,
    pub weight: f64,
}

pub struct Naive {
    remaining: Cow<'static, Vec<(Word, usize)>>,
    patterns: Cow<'static, Vec<[Correctness; 5]>>,
}

impl Naive {
    pub fn new() -> Self {
        Naive {
            remaining: Cow::Borrowed(INITIAL.get_or_init(|| {
                Vec::from_iter(DICTIONARY.lines().map(|line| {
                    let (word, count) = line
                        .split_once(" ")
                        .expect("Every line is Word + Space + Count");

                    let count = count.parse().expect("Count is not a number");
                    let word = word.as_bytes().try_into().unwrap();

                    (word, count)
                }))
            })),
            patterns: Cow::Borrowed(PATTERN.get_or_init(Correctness::compose)),
        }
    }
}

impl Guesser for Naive {
    fn guess(&mut self, history: &[Guess]) -> Word {
        if history.is_empty() {
            return "crate".as_bytes().try_into().unwrap();
        }
        // compute the next POSSIBLE words based on the correctness of the last Guess
        if let Some(last) = history.last() {
            if matches!(self.remaining, Cow::Owned(_)) {
                self.remaining.to_mut().retain(|(w, _)| last.matches(*w));
            } else {
                self.remaining = Cow::Owned(
                    self.remaining
                        .iter()
                        .filter(|(w, _)| last.matches(*w))
                        .copied()
                        .collect(),
                );
            }
        }

        // compute the Shannon Measure of Information(SMI) for the remaining POSSIBLE words
        // ...and we return word with the highest weight. See below

        // SMI = -SUM over all possible patterns of [(p_word(that match a give pattern) * log(p_word))]
        let total_remaining = self.remaining.iter().map(|(_, count)| count).sum::<usize>() as f64;

        let mut best: Option<Candidate> = None;
        for &(word, count) in &*self.remaining {
            let mut expected_information = 0.0;

            let check_matches = |pattern: &[Correctness; 5]| {
                // this is the total frequency of word that matches pattern
                let mut frequency_word_match: usize = 0;
                for &(candidate, count) in &*self.remaining {
                    if (Guess {
                        word,
                        mask: *pattern,
                    }
                    .matches(candidate))
                    {
                        frequency_word_match += count;
                    }
                }
                if frequency_word_match == 0 {
                    return false;
                }
                /*
                * p_word(over a given Pattern) =
                (total frequecy of words that matches that patterns)
                ----------------------------------------------------
                (total frequency of words in search space)
                */
                let p_word = frequency_word_match as f64 / total_remaining as f64;
                expected_information += -p_word * p_word.log2();
                return true;
            };

            if matches!(self.patterns, Cow::Owned(_)) {
                self.patterns.to_mut().retain(check_matches);
            } else {
                self.patterns = Cow::Owned(
                    self.patterns
                        .iter()
                        .copied()
                        .filter(check_matches)
                        .collect(),
                )
            }

            /*
            Applying weight to the expected_information based on the likelihood of each word;

                Suppose 'X' and 'Y' are the words we want to choose between,
               .. such that the expected_information in both is as follows:

               expected_information of X = 2.5 bits
               expected_information of Y = 2.5 bits

               Now, suppose the
               p(X) = 0.5 and p(Y) = 0.2, then;
               weight = p(word) * expected_information

            Hence the higher weight is more likely to be the next guess,
            ... however, if it isn't the answer, and it has a lower expected_information compared
            ... with others, then it means that the number of guesses to reach may likely increase
            * */
            let p_word = count as f64 / total_remaining;
            let weight = p_word * expected_information;
            match best {
                Some(c) => {
                    if weight > c.weight {
                        best = Some(Candidate { word, weight });
                    }
                }
                None => {
                    best = Some(Candidate { word, weight });
                }
            }
        }
        best.unwrap().word
    }
}
