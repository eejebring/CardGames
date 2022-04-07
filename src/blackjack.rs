use crate::card_collection::CardStack;

pub fn startBlackjack() {
    let mut deck = CardStack::newDeck();
    deck.shuffle();
    /*for c in deck.iter() {
        println!("{}",c.to_string());
    }*/
    let mut player_hand = CardStack::newHand();
    let mut house_hand = CardStack::newHand();

    println!("You where dealt: {}", player_hand.receive_card(&mut deck));
    println!("The house where dealt an unknown card");
    house_hand.receive_card(&mut deck);
    println!("You where dealt: {}", player_hand.receive_card(&mut deck));
    println!("The house where dealt: {}",house_hand.receive_card(&mut deck));
    println!("your card sum is {}", sumDeck(&player_hand))
}

pub trait blackjackSum {
    fn blackSum(&self) -> usize;
}

fn sumDeck (cards: &dyn blackjackSum) -> usize {
    cards.blackSum()
}