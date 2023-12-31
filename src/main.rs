#[allow(unused_imports)]
use wordul::algorithm::Naive;
use wordul::Wordle;
const GAMES: &str = include_str!("../answers.txt");

fn main() {
    #[allow(unused_variables)]
    let w = Wordle::new();

    for answer in GAMES.split_whitespace() {
        let guesser = Naive::new();

        if let Some(c) = w.play(answer, guesser) {
            println!("Guessed {} in {}", answer, c);
        } else {
            println!("Failed to make a Guess");
        }
    }
}
