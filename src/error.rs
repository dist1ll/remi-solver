use std::fmt;

use crate::Card;

#[derive(Debug, Clone)]
pub enum Error {
    /// Card of given type is not contained in the deck.
    CardNotContained(Card),

    ParseError,
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::CardNotContained(_) => "No cards of type left",
            Error::ParseError => "Error parsing Card/Hand",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CardNotContained(c) => write!(f, "No cards of type {:?} left", c),
            Error::ParseError => write!(f, "Couldn't parse Card or Hand string"),
        }
    }
}
