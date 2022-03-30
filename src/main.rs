use crate::console_querry::{Query};

mod console_querry;

fn main() {
    let question = Query {
        question: "Pick a game! \n  1. blackjack",
        fail: "That is not a valid option!\n Please try again:"
    };

    println!("{}",question.query_int());


}
