use crate::card::*;
use crate::gamedef::*;

use super::decomp::*;

/// Relative score function to evaluate strength of
/// decompositions. A decomp with a higher score should
/// have a higher chance to win. Depends on remaining cards
/// in the deck.
pub fn decomp_score(p: &Partition, d: &Deck) -> f64 {
    let mut score: f64 = 0.0;
    for g in p.iter() {
        if !g.is_meld() {
            score += 0.05 * g.len() as f64;
        }else {
            score += g.len() as f64;
        }
    }
    score
}

#[test]
fn more_melds() {
    let diff = cmp_decomp(
        &Hand::parse_sorted("Ac 2c 3c 4c 4d 4h").unwrap(),
        &[&[0, 1, 2, 3], &[4, 5]], // worse
        &[&[0, 1, 2], &[3, 4, 5]], // better
        &Deck::new(),
    );
    assert!(diff < 0.0);
}

/// Returns the signed score difference of the two given 
/// decompositions `f(d1) - f(d2)`
fn cmp_decomp(h: &Hand, d1: &[&[usize]], d2: &[&[usize]], deck: &Deck) -> f64 {
    let p1 = partition_index(h, d1).unwrap();
    let p2 = partition_index(h, d2).unwrap();

    let score1 = decomp_score(&p1, deck);
    let score2 = decomp_score(&p2, deck);

    score1 - score2
}
