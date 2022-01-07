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

pub trait PartitionUtil {
    /// Removes empty groups from partition
    fn clean_empty(&mut self);
}

impl<'a> PartitionUtil for Partition<'a> {
    fn clean_empty(&mut self) {
        let mut i: usize = 0;
        while i < self.len() {
            if self[i].is_empty() {
                self.swap_remove(i);
                continue;
            }
            i += 1;
        }
    }
}

pub trait GroupCharacteristics {
    /// Returns true if Group has only one element.
    fn is_single(&self) -> bool;
    /// Returns true if Group has at least 3 connected
    /// components = meld.
    fn is_meld(&self) -> bool;
    /// Returns true if Group has exactly 2 connected
    /// components = quasi-meld.
    fn is_quasi_meld(&self) -> bool;
}

impl<'a> GroupCharacteristics for Group<'a> {
    fn is_single(&self) -> bool {
        self.len() == 1
    }
    fn is_quasi_meld(&self) -> bool {
        if self.len() != 2 {
            return false;
        }
        self[0].is_predecessor(self[1])
    }
    fn is_meld(&self) -> bool {
        if self.len() != 3 {
            return false;
        }
        self[0].is_predecessor(self[1]) && self[1].is_predecessor(self[2])
    }
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

/// Finds the optimal partition w.r.t. decomp score (see README).
/// Compute-intensive method, so use sparingly.
pub fn optimal_decomposition(h: &Hand) -> Partition {
    let mut p = naive_decomposition(h);
    let dup_p = p.clone();
    for x in 0..p.len() {
        // skip (quasi-melds)
        if !p[x].is_single() {
            continue;
        }
        for y in 0..p.len() {
            if !p[y].is_single() {
                continue;
            }
            if p[x][0].n == p[y][0].n {
                p[x].try_push(dup_p[y][0]);
                p[y].swap_remove(0);
            }
        }
    }
    p.clean_empty();
    p
}
