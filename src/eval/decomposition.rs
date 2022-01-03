use crate::*;
use arrayvec::ArrayVec;

/// Maximum number of cardgroups (hand) in a decomposition.
pub const MAX_DECOMP_COUNT: usize = 15;

pub type Partition<'a> = ArrayVec<Group<'a>, MAX_DECOMP_COUNT>;

pub type Group<'a> = ArrayVec<&'a Card, MAX_HAND_SIZE>;

pub trait Decomposition {
    fn from_hand();
}

fn decompose_group(g: &Group) {}

#[test]
fn decomp_create() {
    let mut d = Partition::new();
    d.try_push(Hand::parse("1d 2d 3d").unwrap());
    d.try_push(Hand::parse("3c 3h 3s").unwrap());
}

#[test]
fn decomp_simple() {
    let mut h = Hand::parse("5d 8h 5c 6c 2c Qs 10d").unwrap();
    h.sort_unstable();
    let x = h
        .iter()
        .filter(|c| c.suit == Suit::Clubs)
        .collect::<Group>();
    handle(&x);
    println!("{:?}", x);
}

fn handle(g: &Group) {}
