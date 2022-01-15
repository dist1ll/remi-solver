use crate::Error;
use crate::card::*;
use crate::gamedef::*;

use super::decomp::*;

/// Relative score function to evaluate strength of decompositions.
/// A decomp with a higher score should have a higher chance to win
/// Depends on remaining cards in the deck.
pub fn decomp_score(p: &Partition, d: &Deck) -> f64 {
    let mut score: f64 = 0.0;
    for g in p.iter() {
        if !g.is_meld() {
            score += 0.05 * g.len() as f64;
        }else {
            score += g.len() as f64;
            score += extension_score(g, d);
        }
    }

    score
}

/// Likelihood of drawing cards within a turn horizon that extend the
/// given meld.
fn extension_score(g: &Group, d: &Deck) -> f64 {
    if !g.is_meld() {
        return 0.0;
    }

    if g.is_same_number() {
        // can't extend ace meld
        if g.len() == 4 {
            return 0.0;
        }
        
    }
    0.0
}

/// For a given numeric meld with 3 members, computes the last
/// missing suit. Excludes joker!
fn remaining_suit(g: &Group) -> Result<Suit, Error>{
    let mut result: u32 = 0;
    for &c in g.iter() {
       result = result | c.suit.to_int(); 
    }
    // invert result and mask off bits > 4 to exclude joker
    result = (!result) & 0b1111;
    Suit::from_int(result)   
}

#[test]
fn test_remaining_suit() {
    let h = Hand::parse_sorted("4c 4d 4h").unwrap();
    let g = Group::from_hand(&h, &[0, 1, 2]).unwrap();
    assert!(remaining_suit(&g).unwrap() == Suit::Spades);
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

/// Here we check if the scoring function considers the ability or
/// inability of a meld to get extended past its numeric bounds.
#[test]
fn meld_extension_score_numeric() {
    let diff = cmp_decomp(
        &Hand::parse_sorted("Ac 2c 3c 4c 5c").unwrap(),
        &[&[0, 1, 2], &[3, 4]], // worse
        &[&[0, 1], &[2, 3, 4]], // better
        &Deck::new(),
    );
    assert!(diff < 0.0);
}


/// Returns the signed score difference of the two given decompositions 
/// `f(d1) - f(d2)`.
fn cmp_decomp(h: &Hand, d1: &[&[usize]], d2: &[&[usize]], deck: &Deck) -> f64 {
    let p1 = partition_index(h, d1).unwrap();
    let p2 = partition_index(h, d2).unwrap();

    let score1 = decomp_score(&p1, deck);
    let score2 = decomp_score(&p2, deck);

    score1 - score2
}
