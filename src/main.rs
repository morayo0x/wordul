#[allow(unused_imports)]
use wordul::algorithm::Naive;
use wordul::Wordle;
const GAMES: &str = include_str!("../answers.txt");

fn main() {
    #[allow(unused_variables)]
    let w = Wordle::new();
    let mut scores = Vec::new();

    for answer in GAMES.split_whitespace() {
        let guesser = Naive::new();

        if let Some(c) = w.play(answer, guesser) {
            println!("Guessed {} in {}", answer, c);
            scores.push(c);
        } else {
            println!("Failed to make a guess for {}", answer);
        }
    }

    let (count, sum) = (scores.len(), scores.iter().sum::<usize>());
    let average = sum as f64 / count as f64;
    println!("Average number of guesses is: {}", average);
}
