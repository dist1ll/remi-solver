use crate::card::*;
use crate::gamedef::*;
use crate::MAX_HAND_SIZE;

use arrayvec::ArrayVec;

/// Maximum number of cardgroups (hand) in a decomposition.
pub const MAX_DECOMP_COUNT: usize = 15;

pub type Partition<'a> = ArrayVec<Group<'a>, MAX_DECOMP_COUNT>;

pub type Group<'a> = ArrayVec<&'a Card, MAX_HAND_SIZE>;

/// Divides a hand into distinct groups of suits, and returns them
/// as a Partition.
fn partition_suit(h: &Hand) -> Partition {
    let mut p = Partition::new();
    for &suit in Suit::iter() {
        let s_group = h.iter().filter(|c| c.suit == suit).collect::<Group>();
        // add non-empty groups
        if s_group.len() != 0 {
            p.try_push(s_group);
        }
    }
    p
}

fn assert_group_eq(g: &Group, hand: &'static str) {
    let h = Hand::parse(hand).unwrap();
    let mut i: usize = 0;
    for &&elem in g.iter() {
        assert_eq!(elem, h[i]);
        i += 1;
    }
}

#[test]
fn test_partition_suit() {
    let mut h = Hand::parse("Ac 6s 9h 10d 5h 3c Kc 7s 9c 4d Jd").unwrap();
    let partition = partition_suit(&h);
    assert!(partition.len() == 4);
    assert_group_eq(&partition[0], "Ac 3c Kc 9c");
    assert_group_eq(&partition[1], "10d 4d Jd");
    assert_group_eq(&partition[2], "9h 5h");
    assert_group_eq(&partition[3], "6s 7s");
}

#[test]
fn decomp_create() {
    let mut d = Partition::new();
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
