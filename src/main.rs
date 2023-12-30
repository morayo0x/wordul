#[allow(unused_imports)]
use wordul::algorithm::Naive;
use wordul::Wordle;
const GAMES: &str = include_str!("../answers.txt");

fn main() {
    #[allow(unused_variables)]
    let w = Wordle::new();

    for answer in GAMES.split_whitespace() {
        let guesser = Naive::new();

        if let Some(score) = w.play(answer, guesser) {
            eprintln!("Score is {}", score);
        } else {
            eprintln!("Failed to guess");
        }
    }
    //let guesser = Naive::
}
