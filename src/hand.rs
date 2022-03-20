use std::cmp::Ordering;

/// Represents the current hand, which includes the player sum and the dealer sum.
/// The ace boolean variable represents whether an ace was used to count as 11.
#[derive(Debug, Copy, Clone, Hash, Eq)]
pub struct Hand {
    pub sum: u8,
    pub ace: bool,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.ace == other.ace && self.sum == other.sum;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return if self.is_bust() && other.is_bust() {
            None
        } else if self.is_bust() {
            Some(Ordering::Less)
        } else if other.is_bust() {
            Some(Ordering::Greater)
        } else {
            self.sum.partial_cmp(&other.sum)
        };
    }
}

impl Hand {
    pub fn new() -> Hand {
        return Hand { sum: 0, ace: false };
    }

    pub fn from(c1: u8, c2: u8) -> Hand {
        return Hand::new().hit(c1).hit(c2);
    }

    pub fn hit(&self, card: u8) -> Hand {
        let new_sum = self.sum + card;
        return if card == 1 && new_sum + 10 <= 21 {
            //use the Ace card as an 11 instead of a 1
            Hand { sum: new_sum + 10, ace: true }
        } else if new_sum > 21 && self.ace {
            //we busted using the Ace as 11, turn it back to a 1
            Hand { sum: new_sum - 10, ace: false }
        } else {
            Hand { sum: new_sum, ace: self.ace }
        };
    }

    pub fn is_bust(&self) -> bool {
        return self.sum > 21;
    }
}

#[cfg(test)]
mod tests {
    use crate::deck::Deck;
    use super::*;

    #[test]
    fn test_hand_with_ace_1st() {
        let hand = Hand::from(1, 2);

        assert_eq!(hand.ace, true);
        assert_eq!(hand.sum, 13);
    }

    #[test]
    fn test_hand_with_ace_2nd() {
        let hand = Hand::from(5, 1);

        assert_eq!(hand.ace, true);
        assert_eq!(hand.sum, 16);
    }

    #[test]
    fn test_hand_with_two_aces() {
        let hand = Hand::from(1, 1);

        assert_eq!(hand.ace, true);
        assert_eq!(hand.sum, 12);
    }

    #[test]
    fn test_hand_with_no_aces() {
        let hand = Hand::from(5, 6);

        assert_eq!(hand.ace, false);
        assert_eq!(hand.sum, 11);
    }

    #[test]
    fn test_hand_with_no_aces_hits_ace() {
        let hand = Hand::from(3, 6).hit(1);

        assert_eq!(hand.ace, true);
        assert_eq!(hand.sum, 20);
    }

    #[test]
    fn test_hand_with_no_aces_hits_ace_too_high() {
        let hand = Hand::from(5, 6).hit(1);

        assert_eq!(hand.ace, false);
        assert_eq!(hand.sum, 12);
    }

    #[test]
    fn test_busted_hand() {
        let hand = Hand::from(10, 6);
        assert_eq!(hand.sum, 16);

        let hit_hand = hand.hit(10);

        assert!(hit_hand.is_bust());
        assert_eq!(hit_hand.sum, 26);
    }

    #[test]
    fn test_not_busted_hand() {
        let hand = Hand::from(10, 6);
        assert_eq!(hand.sum, 16);

        let hit_hand = hand.hit(2);

        assert!(!hit_hand.is_bust());
        assert_eq!(hit_hand.sum, 18);
    }

    #[test]
    fn test_blackjack_with_ace_1st() {
        let hand = Hand::from(1, 10);
        assert_eq!(hand.sum, 21);
        assert!(!hand.is_bust());
    }

    #[test]
    fn test_blackjack_with_ace_2nd() {
        let hand = Hand::from(10, 1);
        assert_eq!(hand.sum, 21);
        assert!(!hand.is_bust());
    }

    #[test]
    fn test_blackjack_no_ace() {
        let hand = Hand::from(10, 5).hit(6);

        assert_eq!(hand.sum, 21);
        assert!(!hand.is_bust());
    }

    #[test]
    fn test_winning_hand() {
        let hand1 = Hand::from(10, 5).hit(6);
        assert_eq!(hand1.sum, 21);
        assert!(!hand1.is_bust());

        let hand2 = Hand::from(5, 7).hit(8);
        assert_eq!(hand2.sum, 20);
        assert!(!hand2.is_bust());

        assert_ne!(hand1, hand2);
        assert!(hand1 > hand2);
        assert!(hand2 < hand1);
    }

    #[test]
    fn test_winning_against_bustedhand() {
        let hand1 = Hand::from(10, 5).hit(8);
        assert_eq!(hand1.sum, 23);
        assert!(hand1.is_bust());

        let hand2 = Hand::from(5, 7).hit(8);
        assert_eq!(hand2.sum, 20);
        assert!(!hand2.is_bust());

        assert_ne!(hand1, hand2);
        assert!(hand1 < hand2);
        assert!(hand2 > hand1);
    }


    #[test]
    fn test_deal_hand() {
        let mut deck = Deck::new_shuffled();

        let card1 = deck.deal().unwrap();
        let card2 = deck.deal().unwrap();
        let hand = Hand::from(card1, card2);

        if (card1 == 1) || (card2 == 1) {
            assert_eq!(hand.ace, true);
            assert_eq!(hand.sum, card1 + card2 + 10);
        } else {
            assert_eq!(hand.ace, false);
            assert_eq!(hand.sum, card1 + card2);
        }
    }
}