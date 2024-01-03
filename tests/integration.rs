//use wordulCow::Borrowed(::matchesatches;)
use wordul::coret;
use wordul::Correctness;
use wordul::Guess;

#[test]
fn impressive() {
    assert_eq!(
        Correctness::compute(*b"hello", *b"hello"),
        coret![C C C C C]
    );
}

#[test]
fn good() {
    assert_eq!(
        Correctness::compute(*b"cigar", *b"mayor"),
        coret![W M W W C]
    );
}

#[test]
fn goo() {
    assert_eq!(
        Correctness::compute(*b"aabcd", *b"abcde"),
        coret![C M M M W]
    );
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
    assert_eq!(
        Correctness::compute(*b"caacc", *b"baaaa"),
        coret![W C C W W]
    );
}

#[test]
fn compute_c() {
    assert_eq!(
        Correctness::compute(*b"aabcd", *b"abcde"),
        coret![C M M M W]
    );
}

#[test]
fn compute_d() {
    assert_eq!(
        Correctness::compute(*b"zapol", *b"abcdp"),
        coret![M W W W M]
    );
}

#[test]
fn matches_correct() {
    let guess = Guess {
        word: *b"aabde",
        mask: coret![C C C C C],
    };

    assert!(guess.matches(*b"aabde"));
}

#[test]
fn matches_wrong() {
    let guess = Guess {
        word: *b"aabde",
        mask: coret![W W W W W],
    };

    assert!(guess.matches(*b"wtypx"));
}

#[test]
fn matches_misplaced() {
    let guess = Guess {
        word: *b"aabde",
        mask: coret![M M M M M],
    };

    assert!(guess.matches(*b"bdaea"));
}

#[test]
fn matches_random() {
    let guess = Guess {
        word: *b"cigar",
        mask: coret![C M M W M],
    };

    assert!(guess.matches(*b"crwig"));
}
