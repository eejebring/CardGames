use crate::card_collection::CardStack;

pub fn startBlackjack() {
    let mut deck = CardStack::newDeck();
    deck.shuffle();
    for c in deck.iter() {
        println!("{}",c.to_string());
    }
}