// This module only contains definitions and features
// that are required for running a solver. It's not intended
// to provide a backend for an actual match - i.e. no notion
// of multi-player, scores, turns.
use core::fmt;
use std::ops::Index;

use arrayvec::ArrayVec;

use crate::card::*;
use crate::error::Error;
use crate::*;

pub type Hand = ArrayVec<Card, MAX_HAND_SIZE>;

pub trait HandUtil {
    /// Fills the Card array with n cards.
    fn fill(&mut self, n: u32);

    /// Parses a space-separated string and generates a hand from it.
    /// Fails if the syntax/card symbols are incorrect.
    fn parse(s: &str) -> Result<Hand, Error>;

    /// Parses a Hand string and returns a sorted collection of card.
    fn parse_sorted(s: &str) -> Result<Hand, Error>;
}

impl HandUtil for Hand {
    fn fill(&mut self, n: u32) {
        for _ in 0..n {
            self.push(Card::random());
        }
    }

    fn parse(s: &str) -> Result<Hand, Error> {
        let mut h = Hand::new();
        let split = s.split(' ');
        for chunk in split {
            let card = Card::parse(chunk)?;
            if h.try_push(card).is_err() {
                return Err(Error::CapacityError(card));
            }
        }
        Ok(h)
    }

    fn parse_sorted(s: &str) -> Result<Hand, Error> {
        let mut h = Hand::parse(s)?;
        h.sort_unstable();
        Ok(h)
    }
}

/// A Deck is the primary source of cards, containing all unique cards n-times,
/// where n is the number of duplicates. For this version of Rummy, n = 2.
pub struct Deck {
    cards: [(Card, u32); UNIQUE_CARDS],
}

impl Deck {
    /// Creates a full deck (108 cards, 2 duplicates + 4 jokers)
    pub fn new() -> Self {
        let mut idx = 0;
        let mut arr = [(JOKER_CARD, 2); UNIQUE_CARDS].map(|mut _c| {
            let n = Value::new(((idx / 4) % MAX_CARD_VAL) + 1);
            let suit = Suit::from_int(1 << (idx % 4)).unwrap();
            idx += 1;
            (Card { n, suit }, DUPLICATE_COUNT)
        });
        // Add all jokers at the end
        arr[arr.len() - 1] = (JOKER_CARD, JOKER_TOTAL);
        Deck { cards: arr }
    }

    /// Removes the given card from the Deck if it's contained.
    /// Returns an error if no copies of the card are contained.
    pub fn remove(&mut self, c: Card) -> Result<(), Error> {
        match self[c] {
            0 => Err(Error::CardNotContained(c)),
            num => {
                self.cards[c.to_index()].1 = num - 1;
                Ok(())
            }
        }
    }

    /// Draws a random card and returns the card if successful.
    pub fn remove_random(&mut self) -> Result<Card, Error> {
        let drawn_card = self.cards[fastrand::usize(..UNIQUE_CARDS)].0;
        match self.remove(drawn_card) {
            Ok(()) => Ok(drawn_card),
            Err(e) => Err(e),
        }
    }

    pub fn odds_to_draw(&self, card: Card) -> f64 {
        let cards_left = self.cards[card.to_index()].1 as f64;
        cards_left / 104.0
    }
}

impl Index<Card> for Deck {
    type Output = u32;
    fn index(&self, index: Card) -> &Self::Output {
        &self.cards[index.to_index()].1
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.cards)
    }
}
