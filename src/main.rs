use wordul::Wordle;
use wordul::algorithm::Naive;

const DICTIONARY: &str = include_str!("../dictionary.txt");

fn main() {
    let w = Wordle::new();
    let guesser = Naive::
}
