extern crate rust_cribbage;

use rust_cribbage::cards::*;

#[test]
fn test_get_long_suit_desc() {
    assert_eq!(get_long_suit_desc(Suit::HEARTS), "Hearts");
    assert_eq!(get_long_suit_desc(Suit::CLUBS), "Clubs");
    assert_eq!(get_long_suit_desc(Suit::SPADES), "Spades");
    assert_eq!(get_long_suit_desc(Suit::DIAMONDS), "Diamonds");
}

#[test]
fn test_get_short_suit_desc() {
    assert_eq!(get_short_suit_desc(Suit::HEARTS), "H");
    assert_eq!(get_short_suit_desc(Suit::CLUBS), "C");
    assert_eq!(get_short_suit_desc(Suit::SPADES), "S");
    assert_eq!(get_short_suit_desc(Suit::DIAMONDS), "D");
}

#[test]
fn test_card_functions() {
    let mut c = PlayingCard::new("4", Suit::HEARTS);
    assert_eq!(c.get_long_description(), "Four of Hearts");
    assert_eq!(c.get_short_description(), "4H");
    assert_eq!(c.get_suit(), Suit::HEARTS);
    assert_eq!(c.get_long_rank_name(), "Four");
    assert_eq!(c.get_short_rank_name(), "4");
    assert_eq!(c.get_long_suit_name(), "Hearts");
    assert_eq!(c.get_short_suit_name(), "H");
    assert_eq!(c.get_rank(), "4");

    c = PlayingCard::new("Q", Suit::SPADES);
    assert_eq!(c.get_long_description(), "Queen of Spades");
    assert_eq!(c.get_short_description(), "QS");
    assert_eq!(c.get_suit(), Suit::SPADES);
    assert_eq!(c.get_long_rank_name(), "Queen");
    assert_eq!(c.get_short_rank_name(), "Q");
    assert_eq!(c.get_long_suit_name(), "Spades");
    assert_eq!(c.get_short_suit_name(), "S");
    assert_eq!(c.get_rank(), "Q");
}

#[test]
#[should_panic]
fn test_card_invalid_rank() {
    let c = PlayingCard::new("88", Suit::HEARTS);
    let d = c.get_long_description();
}
