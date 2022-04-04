use crate::blackjack::startBlackjack;
use crate::console_querry::{Query, OptionQuery};

mod console_querry;
mod blackjack;
mod cards;
mod card_collection;

fn main() {

    let main_menu = OptionQuery::new("what game would you like to play?", vec![
        "Blackjack".to_string(),
        "21".to_string(),
        "None (Exit)".to_string()
    ]);

    loop {
        match main_menu.query() {
            1 => startBlackjack(),
            2 => println!("WIP, next time maybe."),
            3 => {
                println!("You are welcome to return at any time!");
                break
            },
            _ => {}
        }
    }

}
