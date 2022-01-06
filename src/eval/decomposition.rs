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

trait Optimize {
    /// Finds the optimal partition w.r.t. decomp score (see README).
    /// Compute-intensive method, so use sparingly.
    fn optimize(&mut self);
}

impl<'a> Optimize for Partition<'a> {
    fn optimize(&mut self) {}
}

/// Divides a hand into distinct groups of suits, and returns them
/// as a Partition.
pub fn partition_suit(h: &Hand) -> Partition {
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
pub fn naive_decomposition(h: &Hand) -> Partition {
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
