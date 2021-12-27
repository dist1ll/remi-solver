// This module contains types relating to playing cards, their values and suits.
use core::fmt;
use std::char;

use crate::*;

pub const JOKER_CARD: Card = Card {
    n: Value(0),
    suit: Suit::Joker,
};

/// Defines the canonical value of a playing card, according to the
/// rules of Rummy. E.g. Five = 5, King = 13, Ace = 1.
///
/// Note that Value is different from the meld value, where a cards
/// value is capped to 10 in the context of the meld.
#[derive(Copy, Clone)]
pub struct Value(u32);

impl Value {
    pub fn new(n: u32) -> Value {
        if n > MAX_CARD_VAL {
            panic!("Card number exceeded 13 (King)")
        }
        Value(n)
    }

    pub fn from_char(c: char) -> Result<Value, Error> {
        match c {
            'X' => Ok(Value::new(0)),
            'A' => Ok(Value::new(1)),
            'J' => Ok(Value::new(11)),
            'Q' => Ok(Value::new(12)),
            'K' => Ok(Value::new(13)),
            num => match num.to_digit(10) {
                None => Err(Error::ParseError),
                Some(n) => Ok(Value::new(n)),
            },
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            0 => write!(f, "X"),
            1 => write!(f, "A"),
            2..=10 => write!(f, "{}", self.0),
            11 => write!(f, "J"),
            12 => write!(f, "Q"),
            13 => write!(f, "K"),
            _ => panic!("Card number exceeded 13 (King)."),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
    Joker,
}

impl Suit {
    pub fn from_int(n: u32) -> Result<Suit, ()> {
        match n {
            0 => Ok(Suit::Clubs),
            1 => Ok(Suit::Diamonds),
            2 => Ok(Suit::Hearts),
            3 => Ok(Suit::Spades),
            4 => Ok(Suit::Joker),
            _ => Err(()),
        }
    }

    pub fn to_int(self) -> u32 {
        match self {
            Suit::Clubs => 0,
            Suit::Diamonds => 1,
            Suit::Hearts => 2,
            Suit::Spades => 3,
            Suit::Joker => 4,
        }
    }

    pub fn from_char(c: char) -> Result<Suit, Error> {
        match c {
            'c' => Ok(Suit::Clubs),
            'd' => Ok(Suit::Diamonds),
            'h' => Ok(Suit::Hearts),
            's' => Ok(Suit::Spades),
            _ => Err(Error::ParseError),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Card {
    pub n: Value,
    pub suit: Suit,
}

impl Card {
    /// Converts a Card to its index in the sorted list of unique cards.
    /// E.g.: Ac = 0, Ad = 1, ..., J = 52
    pub fn to_index(&self) -> usize {
        match self.suit {
            Suit::Joker => UNIQUE_CARDS - 1,
            _ => {
                let idx: u32 = (self.n.0 - 1) * 4 + self.suit.to_int();
                idx.try_into().unwrap()
            }
        }
    }

    /// Returns a Card generated from the given canonical index.
    /// (The index lies between 0...52 incl.)
    pub fn from_index(i: u32) -> Card {
        let n = Value::new((i / 4) + 1);
        let suit = Suit::from_int(i % 4).unwrap();
        Card { n, suit }
    }

    pub fn parse(s: &str) -> Result<Card, Error> {
        if s == "X" {
            return Ok(JOKER_CARD);
        }
        assert!(s.chars().count() == 2);

        let mut c_iter = s.chars();
        let r_n = Value::from_char(c_iter.next().unwrap());
        let r_suit = Suit::from_char(c_iter.next().unwrap());
        match (r_n, r_suit) {
            (Ok(n), Ok(suit)) => Ok(Card { n, suit }),
            _ => Err(Error::ParseError),
        }
    }

    /// Returns a randomly generated Card.
    pub fn random() -> Card {
        let bound: u32 = UNIQUE_CARDS.try_into().unwrap();
        Card::from_index(fastrand::u32(..bound))
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}{:?}]", self.n, self.suit)
    }
}

impl fmt::Debug for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Suit::Clubs => write!(f, "♣"),
            Suit::Diamonds => write!(f, "♦"),
            Suit::Hearts => write!(f, "♥"),
            Suit::Spades => write!(f, "♠"),
            Suit::Joker => write!(f, ""),
        }
    }
}
