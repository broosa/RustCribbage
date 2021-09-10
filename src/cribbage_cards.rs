use crate::cards::*;
use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CribbageCard(PlayingCard);

impl Card for CribbageCard {
    fn new(rank: &'static str, suit: Suit) -> CribbageCard {
        CribbageCard(PlayingCard::new(rank, suit))
    }

    fn get_suit(&self) -> Suit {
        self.0.get_suit()
    }

    fn get_rank(&self) -> &'static str {
        self.0.get_rank()
    }

    fn get_value(&self) -> i8 {
        match self.get_rank() {
            "J" | "Q" | "K" => 10,
            _ => self.0.get_value(),
        }
    }
}

impl CribbageCard {
    pub fn directly_precedes(&self, other: Self) -> bool {

        if self.get_rank() == "K" {
            return false;
        }
        let this_position = RANKS.iter().position(|&x| x == self.get_rank()).unwrap();
        let other_position = RANKS.iter().position(|&x| x == other.get_rank()).unwrap();

        other_position - this_position == 1
    }

    pub fn directly_succeeds(&self, other: Self) -> bool {
        if self.get_rank() == "A" {
            return false;
        }
        let this_position = RANKS.iter().position(|&x| x == self.get_rank()).unwrap();
        let other_position = RANKS.iter().position(|&x| x == other.get_rank()).unwrap();

        this_position - other_position == 1
    }
}

impl fmt::Display for CribbageCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_short_description())
    }
}





