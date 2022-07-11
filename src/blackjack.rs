use crate::card_collection::CardStack;
use crate::OptionQuery;

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

    let more = OptionQuery::new("would you like another card?",vec![
        "Hit".to_string(),
        "Cut".to_string()
    ]);

    loop {
        let sum = sumDeck(&player_hand);
        println!("your card sum is {}", sum);
        if 21 < sum {
            println!("bust!");
            break;
        }
        match more.query() {
            1 => println!("You where dealt: {}", player_hand.receive_card(&mut deck)),
            _ => break
        }
    }


    println!("The house reveals its hidden card: {}",house_hand.iter().next().unwrap().to_string());
    loop {
        let sum = sumDeck(&house_hand);
        if sum < 17 {
            println!("The house was dealt: {}",house_hand.receive_card(&mut deck));
        }
        else if 21 < sum {
            println!("house card sum is {}{}",sum,"\n Bust!");
            break;
        }
        else {
            println!("house card sum is {}",sum);
            if sum < sumDeck(&player_hand) {
                println!("Player wins!")
            }
            else {
                println!("House wins!");
            }
            break;
        }
    }
}

pub trait blackjackSum {
    fn blackSum(&self) -> usize;
}

fn sumDeck (cards: &dyn blackjackSum) -> usize {
    cards.blackSum()
}