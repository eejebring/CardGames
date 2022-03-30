use crate::console_querry::{Query};

mod console_querry;

fn main() {
    let question = Query {
        question: "Pick a game! \n 1. blackjack",
        fail: "I said number!"
    };

    println!("{}",question.query_int());
}
