use std::fmt;

use crate::Card;

#[derive(Debug, Clone)]
pub enum Error {
    /// Card of given type is not contained in the deck.
    CardNotContained(Card),

    ParseError,

    CapacityError(Card),

    GroupIndexError,
}

impl ::std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CardNotContained(c) => write!(f, "No cards of type {:?} left", c),
            Error::ParseError => write!(f, "Couldn't parse Card or Hand string"),
            Error::CapacityError(c) => write!(f, "Can't add card {:?}, hand is full", c),
            Error::GroupIndexError => write!(f, "Incorrect index given for group"),
        }
    }
}
