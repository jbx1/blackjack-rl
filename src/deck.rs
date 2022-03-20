extern crate rand;

use std::collections::VecDeque;
use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
pub struct Deck {
    cards: VecDeque<u8>
}

impl Deck {
    /// returns a new ordered deck, face cards are represented by a ten (so each deck will contain 16 tens)
    pub fn new() -> Deck {
        return Deck{ cards: VecDeque::from(Deck::init_cards())};
    }

    /// returns a new shuffled deck
    pub fn new_shuffled() -> Deck {
        return Deck{ cards: VecDeque::from(Deck::init_cards_shuffled())};
    }

    /// returns a new deck containing the specified cards in the specified order (for testing purposes)
    pub fn new_rigged(cards: &[u8]) -> Deck {
        let mut deque = VecDeque::new();
        deque.extend(cards.iter().copied());

        return Deck{ cards : deque };
    }

    /// returns a vector of all cards in a deck shuffled
    fn init_cards_shuffled() -> Vec<u8> {
        let mut cards = Deck::init_cards();
        cards.shuffle(&mut thread_rng());
        return cards;
    }

    /// returns a vector of all cards in a deck in order
    fn init_cards() -> Vec<u8> {
        let mut cards : Vec<u8> = Vec::new();

        for _ in 1..=4 {
            for card in 1..=10 {
                if card == 10 {
                    //there are 3 face cards apart from a 10 that also count as a 10 for each suit
                    for _ in 1..=4 {
                        cards.push(card);
                    }
                }
                else {
                    cards.push(card);
                }
            }
        }

        return cards;
    }

    /// deals a card from the deck
    pub fn deal(&mut self) -> Option<u8> {
        return self.cards.pop_front();
    }

    /// shuffles the deck in-place
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        let mut i = self.cards.len();
        while i > 1 {
            i -= 1;
            //choose an element to swap with the last one randomly
            let v = rng.gen_range(0..i + 1);
            self.cards.swap(i, v);
        }
    }

    /// returns how many cards are left
    pub fn len(&self) -> usize {
        return self.cards.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_size() {
        let deck = Deck::new();

        assert_eq!(52, deck.len());
    }

    #[test]
    fn test_new_deck_cards() {
        let deck = Deck::new();

        for (i, card) in deck.cards.iter().enumerate() {
            let rem = (i % 13) + 1;
            let expected = if rem > 10 { 10 } else { rem } as u8;

            assert_eq!(expected, *card);
        }
    }

    #[test]
    fn test_new_rigged_deck() {
        let cards:[u8; 5] = [1, 2, 3, 4, 5];
        let mut deck = Deck::new_rigged(&cards);

        assert_eq!(deck.len(), cards.len());
        for i in 0 .. cards.len() {
            assert_eq!(deck.deal().unwrap(), cards[i]);
        }
    }

    #[test]
    fn test_deal() {
        let mut deck = Deck::new_shuffled();
        let next = deck.cards.front().unwrap();
        let card = *next;

        let dealt_card = deck.deal();
        assert!(dealt_card.is_some());
        assert_eq!(card, dealt_card.unwrap());
    }

    #[test]
    fn test_new_shuffled_deck() {
        let deck = Deck::new();
        let shuffled = Deck::new_shuffled();

        let out_of_place = count_out_of_place(&deck.cards, &shuffled.cards);
        assert!(out_of_place > 0);
    }

    #[test]
    fn test_deck_in_place_shuffling() {
        let deck = Deck::new();
        let mut shuffled = Deck::new();
        shuffled.shuffle();

        let out_of_place = count_out_of_place(&deck.cards, &shuffled.cards);
        assert!(out_of_place > 0);
    }

    fn count_out_of_place(v1 : &VecDeque<u8>, v2: &VecDeque<u8>) -> usize {
        let mut out_of_place = 0;
        for i in 0 .. v1.len() {
            if v1[i] != v2[i] {
                out_of_place += 1;
            }
        }
        println!("Shuffled deck has {} out of place", out_of_place);
        return out_of_place;
    }
}



