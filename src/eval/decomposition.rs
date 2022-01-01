use crate::*;
use arrayvec::ArrayVec;

/// Maximum number of cardgroups (hand) in a decomposition.
pub const MAX_DECOMP_COUNT: usize = 15;

pub type Decomp = ArrayVec<Hand, MAX_DECOMP_COUNT>;

pub trait Decomposition {
    fn from_hand();
}

#[test]
fn decomp_create() {
    let mut d = Decomp::new();
    d.try_push(Hand::parse("1d 2d 3d").unwrap());
    d.try_push(Hand::parse("3c 3h 3s").unwrap());
}

#[test]
fn decomp_simple() {
    println!("\npartioning testing:\n");
}
