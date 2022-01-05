use crate::card::*;
use crate::gamedef::*;
use crate::MAX_HAND_SIZE;

use arrayvec::ArrayVec;

/// Maximum number of cardgroups (hand) in a decomposition.
pub const MAX_DECOMP_COUNT: usize = 15;

pub type Partition<'a> = ArrayVec<Group<'a>, MAX_DECOMP_COUNT>;

/// A Group is a subset of Cards. A group may or may not contain
/// melds or quasi-melds.
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

/// A naive decomposition is a simple, suboptimal partition that
/// groups cards into melds or quasi-melds.
///
/// It can be used to bootstrap an optimal decomposition search.
fn naive_decomposition(h: &Hand) -> Partition {
    let p = partition_suit(h);
    let mut decomp = Partition::new();
    for sg in p.iter() {
        let mut current_group = Group::new();
        for &c in sg.iter() {
            match current_group.last() {
                None => current_group.try_push(&c).unwrap(),
                Some(&card) => {
                    if card.is_predecessor(c) {
                        current_group.try_push(&c).unwrap()
                    } else {
                        decomp.try_push(current_group.clone());
                        current_group = Group::new();
                        current_group.try_push(&c).unwrap();
                    }
                }
            };
        }
        if !current_group.is_empty() {
            decomp.try_push(current_group);
        }
    }
    decomp
}
///////////////////
//  UNIT TESTING //
///////////////////

/// Asserts whether the cards in a Group are equal to those
/// given in the string argument.
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
    let mut h = Hand::parse("Ac 6s 9h 10d 5h 3c Kc 7s 9c 4d Jd X").unwrap();
    let partition = partition_suit(&h);
    assert!(partition.len() == 5);
    assert_group_eq(&partition[0], "Ac 3c Kc 9c");
    assert_group_eq(&partition[1], "10d 4d Jd");
    assert_group_eq(&partition[2], "9h 5h");
    assert_group_eq(&partition[3], "6s 7s");
    assert_group_eq(&partition[4], "X");
}

#[test]
fn decomp_create() {
    let mut d = Partition::new();
}

#[test]
fn decomp_simple() {
    let mut h = Hand::parse("Ac 2c 3c 2h 4h 5h Qs Ks 8c 9c 10c Qc Kc").unwrap();
    h.sort_unstable();
    let d = naive_decomposition(&h);
    assert!(d.len() == 6);
    assert_group_eq(&d[0], "Ac 2c 3c");
    assert_group_eq(&d[1], "8c 9c 10c");
    assert_group_eq(&d[2], "Qc Kc");
    assert_group_eq(&d[3], "2h");
    assert_group_eq(&d[4], "4h 5h");
    assert_group_eq(&d[5], "Qs Ks");
}

fn handle(g: &Group) {}
