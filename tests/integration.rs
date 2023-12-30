//use wordul::possible_matchesatches;
use wordul::coret;
use wordul::Correctness;
use wordul::Guess;

#[test]
fn impressive() {
    assert_eq!(Correctness::compute("hello", "hello"), coret![C C C C C]);
}

#[test]
fn good() {
    assert_eq!(Correctness::compute("cigar", "mayor"), coret![W M W W C]);
}

#[test]
fn goo() {
    assert_eq!(Correctness::compute("aabcd", "abcde"), coret![C M M M W]);
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
fn possible_matches_correct() {
    let guess = Guess {
        word: "aabde".to_string(),
        mask: coret![C C C C C],
    };

    assert!(guess.possible_matches("aabde"));
}

#[test]
fn possible_matches_wrong() {
    let guess = Guess {
        word: "aabde".to_string(),
        mask: coret![C C W W M],
    };

    assert!(guess.possible_matches("aafce"));
}
#[test]
fn possible_matches_misplaced() {
    let guess = Guess {
        word: "aabde".to_string(),
        mask: coret![M M M M M],
    };

    assert!(guess.possible_matches("ebada"));
}
#[test]
fn possible_matches_random() {
    let guess = Guess {
        word: "baaaa".to_string(),
        mask: coret![M M M M M],
    };

    assert!(guess.possible_matches("aaaba"));
}

#[test]
fn possible_matches_l() {
    let guess = Guess {
        word: "baaaa".to_string(),
        mask: coret![W C M W W],
    };

    assert!(guess.possible_matches("caacc"));
}

#[test]
fn possible_matches_k() {
    let guess = Guess {
        word: "aabcd".to_string(),
        mask: coret![C W M W M],
    };

    assert!(!guess.possible_matches("abdea"));
}

#[test]
fn compute_b() {
    assert_eq!(Correctness::compute("caacc", "baaaa"), coret![W C C W W]);
}

#[test]
fn compute_c() {
    assert_eq!(Correctness::compute("aabcd", "abcde"), coret![C M M M W]);
}

#[test]
fn compute_d() {
    assert_eq!(Correctness::compute("zapol", "abcdp"), coret![M W W W M]);
}

#[test]
fn possible_matches_f() {
    let guess = Guess {
        word: "cigar".to_string(),
        mask: coret![C W M W M],
    };

    // Note the negation symbol
    assert!(!guess.possible_matches("carog"));
}
