use super::decomp::*;
use crate::card::*;
use crate::gamedef::*;

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
    let p = partition_suit(&h);

    assert!(p.len() == 5);
    partition_eq(
        &p,
        "[[[Ac], [3c], [Kc], [9c]], [[10d], [4d], [Jd]], [[9h], [5h]], [[6s], [7s]], [[X]]]",
    );
}

#[test]
fn decomp_naive() {
    let h = Hand::parse_sorted("2c Ac 3c 2h 4h 5h Qs Ks 8c 9c 10c Qc Kc").unwrap();
    let d = naive_decomposition(&h);

    assert!(d.len() == 6);
    partition_eq(&d, "[[[Ac], [2c], [3c]], [[8c], [9c], [10c]], [[Qc], [Kc]], [[2h]], [[4h], [5h]], [[Qs], [Ks]]]");
}

#[test]
fn decomp_optimize_simple() {
    let h = Hand::parse_sorted("5s 5h 5c").unwrap();
    let mut p: Partition = optimal_decomposition(&h);
    partition_eq(&p, "[[[5c], [5h], [5s]]]");
}

#[test]
fn decomp_optimize_simple2() {
    let h = Hand::parse_sorted("Ac 2c 3c 5s 5h 5c").unwrap();
    let mut p: Partition = optimal_decomposition(&h);
    partition_eq(&p, "[[[Ac], [2c], [3c]], [[5c], [5h], [5s]]]");
}

#[test]
fn decomp_optimize_double() {
    let h = Hand::parse_sorted("Ac 2c 3c 4c 4c").unwrap();
    let mut p: Partition = optimal_decomposition(&h);
    partition_eq(&p, "[[[Ac], [2c], [3c], [4c]], [[4c]]]");
}

#[test]
fn decomp_optimize_split_simple() {
    let h = Hand::parse_sorted("Ac 2c 3c 4c 4d 4h").unwrap();
    let mut p: Partition = optimal_decomposition(&h);
    partition_eq(&p, "[[[Ac], [2c], [3c]], [[4c], [4d], [4h]]]");
}
