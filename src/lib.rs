// This crate is a solver for a card game known as 'Otvoreni Remi',
// a variation of the Rummy card matching game originating from the Balkan.
// It's closest to German Rummy, except that the total joker count is 4,
// max hand size is 15, minimum of 51 for the first meld, and slightly
// different meld values.
//
// This crate models only the partial information of a game from the POV
// of a player, i.e. we think in terms of current hand, discarded cards,
// and cards laid off on the table.

// Constants describing the default configuration of a remy deck.
// All invariants of the constants are asserted by unit tests.
pub const MAX_CARD_VAL: u32 = 13;
pub const MAX_HAND_SIZE: usize = 15;
pub const UNIQUE_CARDS: usize = 53;
pub const DUPLICATE_COUNT: u32 = 2;
pub const JOKER_TOTAL: u32 = 4;

mod card;
mod error;
mod eval;
mod gamedef;

pub use crate::card::*;
pub use crate::error::Error;
pub use crate::eval::*;
pub use crate::gamedef::*;
