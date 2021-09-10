use std::collections::VecDeque;
use std::fmt;

use rand::seq::SliceRandom;
use rand::thread_rng;

const USE_UNICODE_SYMBOLS: bool = true;

pub const SUIT_SYM_HEARTS: &'static str = "\u{2665}";
pub const SUIT_SYM_SPADES: &'static str = "\u{2660}";
pub const SUIT_SYM_DIAMONDS: &'static str = "\u{2666}";
pub const SUIT_SYM_CLUBS: &'static str = "\u{2663}";

pub const SUITS: [Suit; 4] = [Suit::HEARTS, Suit::SPADES, Suit::DIAMONDS, Suit::CLUBS];
pub const RANKS: [&'static str; 13] = [
    "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Suit {
    HEARTS,
    SPADES,
    DIAMONDS,
    CLUBS,
}

#[derive(Debug, Copy, Eq, PartialEq, Clone)]
pub struct PlayingCard {
    suit: Suit,
    rank: &'static str,
}

#[derive(Debug, Clone)]
pub struct BasicCardCollection<T: Card> {
    cards: Vec<T>,
}

#[derive(Debug, Clone)]
pub struct BasicCardDeck<T: Card> {
    cards: VecDeque<T>,
}

pub fn get_long_suit_desc(s: Suit) -> &'static str {
    match s {
        Suit::HEARTS => "Hearts",
        Suit::SPADES => "Spades",
        Suit::CLUBS => "Clubs",
        Suit::DIAMONDS => "Diamonds",
    }
}

pub fn get_long_rank_desc(r: &str) -> &'static str {
    match r {
        "A" => "Ace",
        "2" => "Two",
        "3" => "Three",
        "4" => "Four",
        "5" => "Five",
        "6" => "Six",
        "7" => "Seven",
        "8" => "Eight",
        "9" => "Nine",
        "10" => "Ten",
        "J" => "Jack",
        "Q" => "Queen",
        "K" => "King",
        _ => panic!("Unknown rank: {}", r),
    }
}

pub fn get_short_suit_desc(s: Suit) -> &'static str {
    if USE_UNICODE_SYMBOLS {
        return match s {
            Suit::HEARTS => SUIT_SYM_HEARTS,
            Suit::SPADES => SUIT_SYM_SPADES,
            Suit::DIAMONDS => SUIT_SYM_DIAMONDS,
            Suit::CLUBS => SUIT_SYM_CLUBS,
        };
    }
    &get_long_suit_desc(s)[..1]
}

pub trait MutableCardCollection<T: Card> {
    fn get_cards(&self) -> &VecDeque<T>;
    fn get_num_cards(&self) -> usize {
        self.get_cards().len()
    }
    fn draw_top(&mut self, n: usize) -> Vec<T>;
    fn draw_bottom(&mut self, n: usize) -> Vec<T>;
    fn draw_at(&mut self, n: usize, loc: usize) -> Vec<T>;
    fn peek_top(&mut self, n: usize) -> Vec<T>;
    fn peek_bottom(&mut self, n: usize) -> Vec<T>;
    fn peek_at(&mut self, n: usize, loc: usize) -> Vec<T>;
    fn insert_top(&mut self, c: T);
    fn insert_bottom(&mut self, c: T);
    fn insert_before(&mut self, c: T, loc: usize);
    fn insert_multiple_top(&mut self, c: Vec<T>);
    fn insert_multiple_bottom(&mut self, c: Vec<T>);
    fn insert_multiple_at(&mut self, c: Vec<T>, loc: usize);
    fn shuffle(&mut self);
}

impl<T: Card> fmt::Display for BasicCardCollection<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = cards_to_string(&self.cards);
        write!(f, "{}", s)
    }
}

impl<T: Card> fmt::Display for BasicCardDeck<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = cards_to_string(&Vec::from(self.cards.to_owned()));
        write!(f, "{}", s)
    }
}

impl<T: Card> MutableCardCollection<T> for BasicCardDeck<T> {
    fn get_cards(&self) -> &VecDeque<T> {
        &self.cards
    }

    fn draw_top(&mut self, n: usize) -> Vec<T> {
        let mut drawn_cards: Vec<T> = Vec::new();
        for _ in 0..n {
            let front_card = self.cards.pop_front();
            match front_card {
                Some(c) => drawn_cards.push(c),
                None => break,
            }
        }
        drawn_cards
    }

    fn draw_bottom(&mut self, n: usize) -> Vec<T> {
        let mut drawn_cards: Vec<T> = Vec::new();
        for _ in 0..n {
            let back_card = self.cards.pop_back();
            match back_card {
                Some(c) => drawn_cards.push(c),
                None => break,
            }
        }
        drawn_cards
    }

    fn draw_at(&mut self, n: usize, loc: usize) -> Vec<T> {
        let mut drawn_cards: Vec<T> = Vec::new();
        for i in 0..n {
            let drawn_card = self.cards.remove(loc + i);
            match drawn_card {
                Some(c) => drawn_cards.push(c),
                None => break,
            }
        }
        drawn_cards
    }

    fn peek_top(&mut self, n: usize) -> Vec<T> {
        let mut drawn_cards: Vec<T> = Vec::new();
        for i in 0..n {
            let drawn_card = self.get_cards().get(i);
            match drawn_card {
                Some(c) => drawn_cards.push(*c),
                None => break,
            }
        }
        drawn_cards
    }

    fn peek_bottom(&mut self, n: usize) -> Vec<T> {
        let mut drawn_cards: Vec<T> = Vec::new();

        for i in 0..n {
            let num_cards = self.get_num_cards();
            let drawn_card = self.cards.get(num_cards - (1 + i));
            match drawn_card {
                Some(c) => drawn_cards.push(*c),
                None => break,
            }
        }

        drawn_cards
    }

    fn peek_at(&mut self, n: usize, loc: usize) -> Vec<T> {
        let mut drawn_cards: Vec<T> = Vec::new();
        for i in 0..n {
            let drawn_card = self.cards.get(loc + i);
            match drawn_card {
                Some(c) => drawn_cards.push(*c),
                None => break,
            }
        }

        drawn_cards
    }

    fn insert_top(&mut self, c: T) {
        self.cards.push_front(c);
    }

    fn insert_bottom(&mut self, c: T) {
        self.cards.push_back(c);
    }

    fn insert_before(&mut self, c: T, loc: usize) {
        self.cards.insert(loc, c);
    }

    fn insert_multiple_top(&mut self, c: Vec<T>) {
        for (i, card) in c.iter().enumerate() {
            self.insert_before(*card, i);
        }
    }

    fn insert_multiple_bottom(&mut self, c: Vec<T>) {
        self.cards.extend(c.iter());
    }

    fn insert_multiple_at(&mut self, c: Vec<T>, loc: usize) {
        for (i, card) in c.iter().enumerate() {
            self.insert_before(*card, loc + i);
        }
    }

    fn shuffle(&mut self) {
        //Convert to vec because VecDeque does not have shuffle :(
        let temp_vec: &mut Vec<T> = &mut Vec::from(self.cards.clone());
        temp_vec.shuffle(&mut thread_rng());
        self.cards = VecDeque::from(temp_vec.clone());
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", get_long_suit_desc(*self))
    }
}

pub trait Card: Copy {
    fn new(rank: &'static str, suit: Suit) -> Self;
    fn get_value(&self) -> i8;
    fn get_rank(&self) -> &'static str;
    fn get_suit(&self) -> Suit;
    fn get_short_rank_name(&self) -> &'static str {
        self.get_rank()
    }

    fn get_long_rank_name(&self) -> &'static str {
        get_long_rank_desc(self.get_rank())
    }

    fn get_short_suit_name(&self) -> &'static str {
        get_short_suit_desc(self.get_suit())
    }

    fn get_long_suit_name(&self) -> &'static str {
        get_long_suit_desc(self.get_suit())
    }

    fn get_short_description(&self) -> String {
        format!(
            "{}{}",
            self.get_rank(),
            get_short_suit_desc(self.get_suit())
        )
    }

    fn get_long_description(&self) -> String {
        format!(
            "{} of {}",
            self.get_long_rank_name(),
            get_long_suit_desc(self.get_suit())
        )
    }
}

impl Card for PlayingCard {
    fn new(rank: &'static str, suit: Suit) -> Self {
        PlayingCard {
            rank: rank,
            suit: suit,
        }
    }

    fn get_value(&self) -> i8 {
        match self.get_rank() {
            "A" => 1,
            "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" => {
                self.get_rank().parse::<i8>().unwrap()
            }
            "J" => 11,
            "Q" => 12,
            "K" => 13,
            _ => panic!("Invalid Card Value: {}", self.get_rank()),
        }
    }

    fn get_rank(&self) -> &'static str {
        self.rank
    }

    fn get_suit(&self) -> Suit {
        self.suit
    }
}

pub fn cards_to_string<T: Card>(cards: &Vec<T>) -> String {
    cards.iter()
            .map(|c| c.get_short_description())
            .collect::<Vec<String>>()
            .join(" ")
}

pub fn create_standard_deck<T: Card>() -> BasicCardDeck<T> {
    let mut cards: VecDeque<T> = VecDeque::new();
    for suit in SUITS.iter() {
        for rank in RANKS.iter() {
            cards.push_back(T::new(rank, *suit));
        }
    }

    BasicCardDeck::<T> { cards: cards }
}
