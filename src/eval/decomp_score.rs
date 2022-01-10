use crate::card::*;
use crate::gamedef::*;

use super::decomp::*;

/// Relative score function to evaluate strength of
/// decompositions. A decomp with a higher score should
/// have a higher chance to win. Depends on remaining cards
/// in the deck.
pub fn decomp_score(p: &Partition, d: &Deck) -> f64 {
    0.0
}

#[test]
fn decomp_score_simple() {
    let h = Hand::parse_sorted("Ac 2c 3c 4c 4d 4h").unwrap();
    let mut g = Group::new();
    g.try_push(&h[0]);
    let mut p: Partition = optimal_decomposition(&h);
    partition_eq(&p, "[[[Ac], [2c], [3c]], [[4c], [4d], [4h]]]");
}
