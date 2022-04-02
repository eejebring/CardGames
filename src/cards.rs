use std::fmt;
use std::fmt::Formatter;

enum card_colour {
    Hearts,
    Diamonds,
    Spades,
    Clovers
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
    colour: card_colour,
    value: u8
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
