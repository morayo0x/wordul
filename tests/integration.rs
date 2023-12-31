//use wordul::matchesatches;
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
        mask: coret![W W W W W],
    };

    assert!(guess.matches("wtypx"));
}

#[test]
fn matches_misplaced() {
    let guess = Guess {
        word: "aabde".to_string(),
        mask: coret![M M M M M],
    };

    assert!(guess.matches("bdaea"));
}

#[test]
fn matches_random() {
    let guess = Guess {
        word: "cigar".to_string(),
        mask: coret![C M M W M],
    };

    assert!(guess.matches("crwig"));
}
