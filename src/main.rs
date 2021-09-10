mod cards;
mod cribbage_cards;
mod cribbage_scoring;

use cards::*;
use cribbage_cards::*;
use cribbage_scoring::*;

fn main() {
    let mut deck = create_standard_deck::<CribbageCard>();
    println!("Before shuffle:"); 
    println!("{}", deck);
    deck.shuffle();
    println!("After Shuffle:");
    println!("{}", deck);
    let hand = deck.draw_top(5);
    let cut_card = deck.draw_top(1)[0];

    println!("Hand: {}", cards_to_string(&hand));
    println!("Sorting hand:");
    let sorted = sort_hand(&hand);
    println!("{}", cards_to_string(&sorted));
    println!("Scoring hand");
    score_hand(&hand, cut_card);

    println!("Deck after draw:");
    println!("{}", deck);
    println!("Hello, world!");
}