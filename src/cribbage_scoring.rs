use crate::cards::*;
use crate::cribbage_cards::*;

pub fn sort_hand(hand: &Vec<CribbageCard>) -> Vec<CribbageCard> {
    let mut result_cards = hand.to_owned();
    result_cards.sort_by(|lhs, rhs| lhs.get_value().cmp(&rhs.get_value()));
    Vec::<CribbageCard>::from(result_cards)
}

fn get_subset_sums(n: i8, cards: &[CribbageCard]) -> Vec<Vec<CribbageCard>> {
    let mut result = Vec::<Vec<CribbageCard>>::new();
    for i in 0..cards.len() {
        let card_value = cards[i].get_value();

        //If card_value has a value of n add cards[i] to the set of results
        if card_value == n {
            result.push(vec![cards[i]]);
            continue;
        }
        
        //If card_value > n we're done with this iteration
        if card_value > n {
            break;
        }

        //If card_value < n, find all subsets in remaining items adding up to n - card_value
        let smaller_sums = get_subset_sums(n - card_value, &cards[i + 1..]);
        for mut sum in smaller_sums {
            sum.push(cards[i]);
            result.push(sum);
        }
    }

    result
}

pub fn score_fifteens(hand: &Vec<CribbageCard>) -> (i8, Vec<Vec<CribbageCard>>) {
    let fifteens = get_subset_sums(15, &hand[..]);
    let score = fifteens.len() as i8 * 2;
    let result = fifteens.clone();
    (score, result)
}

pub fn score_quads(hand: &Vec<CribbageCard>) -> (i8, Vec<Vec<CribbageCard>>, Vec<CribbageCard>) {

    let mut results = Vec::<Vec<CribbageCard>>::new();
    let mut remainder = Vec::<CribbageCard>::new();
    let mut score = 0;

    assert_eq!(hand.len(), 5);

    if hand[0].get_rank() == hand[3].get_rank() {
        results.push(vec![hand[0], hand[1], hand[2], hand[3]]);
        remainder.push(hand[4]);
        score = 12;
    }

    if hand[1].get_rank() == hand[4].get_rank() {
        results.push(vec![hand[1], hand[2], hand[3], hand[4]]);
        remainder.push(hand[0]);
        score = 12;
    }

    if results.len() == 0 {
        remainder = hand.to_vec();
    }

    (score, results, remainder)
}


pub fn score_triples(hand: &Vec<CribbageCard>) -> (i8, Vec<Vec<CribbageCard>>, Vec<CribbageCard>) {
    let mut results = Vec::<Vec<CribbageCard>>::new();
    let mut remainder = Vec::<CribbageCard>::new();
    let score = 0;

    //If we've been presented with less than 3 cards, its because a larger hand had a
    //Quad in it.
    if hand.len() < 3 {
        return (0, results, hand.to_vec())
    }

    assert!(hand[0] != hand[3] && hand[1] != hand[4], "tried to score triples on hand with a quad");

    let mut triple_start = 0;
    let mut has_triple = false;

    for i in 0..(hand.len() - 2) {
        if hand[i].get_rank() == hand[i + 2].get_rank() {
            triple_start = i;
            has_triple = true;
            break;
        }
    }

    //Add cards not in the triple to the remainder. should be at most two cards 
    if has_triple {
        for j in 0..triple_start {
            remainder.push(hand[j]);
        }

        for k in (triple_start + 3)..hand.len() {
            remainder.push(hand[k]);
        }

        results.push(vec![hand[triple_start], hand[triple_start + 1], hand[triple_start+ 2]]);
        return (3, results, remainder)
    }

    (0, results, hand.to_vec())
}

pub fn score_pairs(hand: &Vec<CribbageCard>) -> (i8, Vec<Vec<CribbageCard>>, Vec<CribbageCard>) {
    let mut results = Vec::<Vec<CribbageCard>>::new();
    let mut remainder = Vec::<CribbageCard>::new();
    let mut score = 0;

    if hand.len() < 2 {
        return (0, results, hand.to_vec());
    }

    let mut i = 0;
    while i < hand.len() {
        if i == hand.len() - 1 {
            remainder.push(hand[i]);
            break;
        }
        //If card i has the same rank as card i + 1, add to results, otherwise add to remainder.
        if hand[i].get_rank() == hand[i + 1].get_rank() {
            results.push(vec![hand[i], hand[i + 1]]);
            score += 2;
            i += 2;
            continue;
        }
        remainder.push(hand[i]);
        i += 1;
    }
    (score, results, remainder)
}

pub fn score_runs(hand: &Vec<CribbageCard>) -> (i8, Vec<Vec<CribbageCard>>, Vec<CribbageCard>) {
    let mut results = Vec::<Vec<CribbageCard>>::new();
    let mut remainder = Vec::<CribbageCard>::new();
    let mut score = 0;

    if hand.len() < 3 {
        return (0, results, hand.to_vec());
    }

    let mut i = 0;
    let mut possible_run = Vec::<CribbageCard>::new();
    while i < hand.len() {
        //If there aren't enough cards left in the hand, just add them to the remainder
        //and go to the next card immediately.
        if hand.len() - i < 3 {
            remainder.push(hand[i]);
            i += 1;
            continue;
        }

        if !hand[i].directly_precedes(hand[i + 1]) {
            remainder.push(hand[i]);
            i+= 1;
            continue;
        }

        possible_run.push(hand[i]);

        //Consume cards until the next one does not directly precede the current card
        while hand[i].directly_precedes(hand[i + 1]) {
            possible_run.push(hand[i + 1]);
            i += 1;
            if i > hand.len() - 2 {
                break;
            }
        }
        
        //Based on the legth of the run either keep it or discard it.
        let l = possible_run.len();
        match l {
            l if l >= 3 => {
                score = possible_run.len();
                results.push(possible_run);
            },
            _ => remainder.extend(possible_run.into_iter()),
        }

        possible_run = Vec::<CribbageCard>::new();
        i += 1;

    }
    (score as i8, results, remainder)
}

pub fn score_flush(hand: &Vec<CribbageCard>) -> (i8, Vec<Vec<CribbageCard>>) {
    let mut result = Vec::<Vec<CribbageCard>>::new();
    for i in 0..hand.len() - 1 {
        if hand[i].get_suit() != hand[i + 1].get_suit() {
            return (0, result)
        }
    }

    result.push(hand.to_vec());
    (5, result)
}

pub fn score_hand(hand: &Vec<CribbageCard>, cut_card: CribbageCard) -> i32 {
    let mut temp_hand = hand.clone();
    temp_hand.push(cut_card);
    let sorted_hand = sort_hand(&temp_hand);
    let (fifteensScore, fifteens) = score_fifteens(&sorted_hand);
    let (quadScore, quads, quads_remainder) = score_quads(&sorted_hand);
    let (tripleScore, triples, triples_remainder) = score_triples(&quads_remainder);
    let (pairScore, pairs, pairs_remainder) = score_pairs(&triples_remainder);
    let (runScore, runs, remainder) = score_runs(&sorted_hand);

    for s in fifteens {
        println!("{}", cards_to_string(&s));
    }
    fifteensScore as i32
}
