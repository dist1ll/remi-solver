use crate::eval::score_hand;

use super::super::*;

#[test]
fn test_fundamental_1() {
    let _c = Card {
        n: Value::new(5),
        suit: Suit::Clubs,
    };
    let mut deck = Deck::new();
    deck.remove(_c).unwrap();

    let mut x = Hand::new();
    x.fill(10);

    println!("Score for hand: {}", score_hand(x, deck));
    println!("hi, i tested something");
}
