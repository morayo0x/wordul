use wordul::algorithm::Naive;
use wordul::ArgParser;
use wordul::Guesser;
use wordul::Implementation;
use wordul::Wordle;
const GAMES: &str = include_str!("../answers.txt");

fn main() {
    let args = ArgParser::parser();
    let (run, cmd) = (args.count, args.implementation);
    match cmd {
        Implementation::Naive => play(Naive::new, run),
        _ => {
            eprintln!("No implementation passed in");
        }
    }
}

fn play<G>(mut mk: impl FnMut() -> G, runs: Option<usize>)
where
    G: Guesser,
{
    let w = Wordle::new();

    for answer in GAMES.split_whitespace().take(runs.unwrap_or(usize::MAX)) {
        let guesser = (mk)();

        if let Some(c) = w.play(answer, guesser) {
            println!("Guessed {} in {}", answer, c);
        } else {
            println!("Failed to make a guess for {}", answer);
        }
    }
}
