use std::fmt;
use std::fmt::Formatter;
use std::slice::Iter;


#[derive(Copy, Clone)]
pub(crate) enum card_colour {
    Hearts,
    Diamonds,
    Spades,
    Clovers
}

impl card_colour {
    pub fn iter() -> Iter<'static, card_colour> {
        static colours: [card_colour;4] = [card_colour::Hearts,card_colour::Diamonds,card_colour::Spades,card_colour::Clovers];
        colours.iter()
    }
}

impl fmt::Display for card_colour {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,
        "{}", match self {
            card_colour::Hearts => "Hearts",
            card_colour::Diamonds => "Diamonds",
            card_colour::Spades => "Spades",
            card_colour::Clovers => "Clovers"
        })
    }
}

pub struct Card {
    pub(crate) colour: card_colour,
    pub(crate) value: u8
}

impl Card {
    pub fn to_string(&self) -> String {
        match self.value {
            1 => format!("Ace of {}",self.colour),
            11 => format!("Knight of {}",self.colour),
            12 => format!("Queen of {}",self.colour),
            13 => format!("King of {}",self.colour),
            _ => {
                if self.value <= 10 && 2 <= self.value {
                    format!("{} of {}",self.value, self.colour)
                }
                else {
                    "Joker".to_string()
                }
            }
        }
    }
}