use crate::blackjack::startBlackjack;
use crate::console_querry::{Query, OptionQuery};

mod console_querry;
mod blackjack;
mod cards;

fn main() {
    let question = Query {
        question: "Give me a number".to_string(),
        fail: "That is not a number!\n Please try again:".to_string()
    };

    println!("{}",question.query_int() * 2);

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
