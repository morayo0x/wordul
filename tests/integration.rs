use wordul::coret;
use wordul::Correctness;
//use wordul::Correctness::{Correct, Misplaced, Wrong};

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
