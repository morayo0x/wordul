#[derive(Debug, Copy, Clone)]
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
    fn guess(&self, history: &[Guess]) -> String;
}

impl Correctness {
    pub fn compute_correctness(answer: &str, guess: &str) {
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
    }
}
