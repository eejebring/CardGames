use std::collections::btree_set::Range;
use std::ops::{Deref, RangeFrom};
use std::slice::Iter;
use rand::prelude::*;
use crate::cards::{Card, card_colour};

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
        for i in (0..self.cards.len()).rev() {
            let num = rnd.gen() % i;
            let temp = self.cards[i];
            self[i] = self[num];
            self[num] = temp;
        }
    }

    pub fn iter(&self) -> Iter<'_, Card> {
        self.cards.iter()
    }
}

impl CardStack {
    fn receive_card(&mut self, c: Card) {
        self.cards.push(c);
    }
}