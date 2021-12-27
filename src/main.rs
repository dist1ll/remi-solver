use remi_solver::*;

fn main() {
    let mut deck = Deck::new();
    println!("{:?}", deck);

    println!(
        "\nI'm drawing a random card: {:?}",
        deck.remove_random().unwrap()
    );
}
