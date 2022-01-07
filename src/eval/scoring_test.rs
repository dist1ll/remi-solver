use crate::eval::score_hand;

use super::super::*;

/// Parses the given arguments into hands and computes scores.
/// Returns true if the first Hand generates a higher score
/// than the second Hand.
fn assert_higher_score(higher: &str, lower: &str, d: &Deck) {
    let h1 = Hand::parse(higher).unwrap();
    let h2 = Hand::parse(lower).unwrap();
    let result = score_hand(&h1, d) > score_hand(&h2, d);
    assert!(
        result,
        "\n\tIncorrect scoring:\n\t\thigher:\t{:?}\n\t\tlower: \t{:?}\n",
        h2, h1
    );
}
