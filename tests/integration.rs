//use wordul::check_matches;
use wordul::coret;
use wordul::Correctness;
use wordul::Guess;

#[test]
fn impressive() {
    assert_eq!(Correctness::compute("hello", "hello"), coret![C C C C C]);
}

#[test]
fn good() {
    assert_eq!(Correctness::compute("clear", "cello"), coret![C M M W W]);
}

#[test]
fn check_coret() {
    let c = [
        Correctness::Correct,
        Correctness::Misplaced,
        Correctness::Wrong,
    ];
    assert_eq!(coret![C M W], c);
}

#[test]
fn matches_correct() {
    let guess = Guess {
        word: "aabde".to_string(),
        mask: coret![C C C C C],
    };

    assert!(guess.matches("aabde"));
}

#[test]
fn matches_wrong() {
    let guess = Guess {
        word: "aabde".to_string(),
        mask: coret![C C W W M],
    };

    assert!(guess.matches("aafce"));
}
#[test]
fn matches_misplaced() {
    let guess = Guess {
        word: "aabde".to_string(),
        mask: coret![M M M M M],
    };

    assert!(guess.matches("ebada"));
}
#[test]
fn matches_random() {
    let guess = Guess {
        word: "baaaa".to_string(),
        mask: coret![M M M M M],
    };

    assert!(guess.matches("aaaba"));
}

#[test]
fn matches_l() {
    let guess = Guess {
        word: "baaaa".to_string(),
        mask: coret![W C M W W],
    };

    assert!(!guess.matches("caacc"));
}

#[test]
fn matches_fd() {
    assert_ne!(Correctness::compute("caacc", "baaaa"), coret![W C M W W]);
}
