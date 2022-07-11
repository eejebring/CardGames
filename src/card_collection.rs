use std::collections::btree_set::Range;
use std::ops::{Deref, RangeFrom};
use std::slice::Iter;
use rand::prelude::*;
use rand::seq::index::IndexVec::USize;
use crate::cards::{Card, card_colour};
use crate::blackjack::blackjackSum;

pub struct CardStack {
    cards: Vec<Card>
}

impl CardStack {
    pub fn newDeck() -> CardStack {
        let mut deck = CardStack {cards: vec![]};
        for c in card_colour::iter() {
            for i in 1..14 {
                deck.cards.push(Card {colour: *c, value: i})
            }
        };
        return deck;
    }

    pub fn newHand() -> CardStack {
        CardStack { cards: vec![] }
    }

    pub fn shuffle(&mut self) {
        //Fisher–Yates shuffle Algorithm
        let mut rnd = rand::thread_rng();
        for i in (2..self.cards.len()).rev() {
            let num:usize = rnd.gen::<usize>() % i;
            let temp:Card = self.cards[i];
            self.cards[i] = self.cards[num];
            self.cards[num] = temp;
        }
    }

    pub fn receive_card(&mut self, cardHolder: &mut CardStack) -> String {
        let card:Card = *cardHolder.cards.first().unwrap();
        cardHolder.cards.remove(0);
        self.cards.push(card);
        card.to_string()
    }

    pub fn give_card(&mut self, cardTaker: &mut CardStack) -> String {
        cardTaker.receive_card(self)
    }

    pub fn iter(&self) -> Iter<'_, Card> {
        self.cards.iter()
    }
}
//update

impl blackjackSum for CardStack {
    fn blackSum(&self) -> usize {
        let mut sum:usize = 0;
        let mut acees = 0;
        for c in &self.cards {
            if c.value == 1 {
                acees += 1;
                sum += c.value as usize;
            }
            else if 10 < c.value {
                sum += 10;
            }
            else {
                sum += c.value as usize;
            }
        };
        loop {
            if sum <= 11 && 1 <= acees {
                println!("found ace");
                acees -= 1;
                sum += 10;
            }
            else  {
                break;
            }
        }
        sum
    }
}
/*
trait TradeCard {
    fn receive_card(&mut self, cardHolder:&mut CardStack) -> String;
    fn give_card(&mut self, cardTaker:&mut dyn TradeCard) -> String;
}

impl TradeCard for CardStack {
    fn receive_card(&mut self, cardHolder: &mut CardStack) -> String {
        let card:Card = *cardHolder.cards.first().unwrap();
        cardHolder.cards.remove(0);
        self.cards.push(card);
        card.to_string()
    }

    fn give_card(&mut self, cardTaker: &mut dyn TradeCard) -> String {
        cardTaker.receive_card(self)
    }
}*/