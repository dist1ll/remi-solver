#[cfg(test)]
mod scoring_test;

use crate::*;

/// Computes an approximate quality score for a given hand and deck.
///
/// The metric is relative, so that if `score(h1,d) > score(h2,d)`, then
/// hand `h2` should have a higher expected winrate.
pub fn score_hand(_h: Hand, _d: Deck) -> f64 {
    0.0
}
