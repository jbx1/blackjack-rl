use crate::round::Outcome::{Draw, Lost, Playing, Won};
use crate::deck::Deck;
use crate::hand::Hand;

#[derive(Debug)]
pub enum Outcome {
    Won,
    Lost,
    Draw,
    Playing,
}

#[derive(Debug)]
pub struct RoundState {
    pub outcome: Outcome,
    pub player: Hand,
    pub dealer: Hand,
    pub hilo: i32,
}

impl RoundState {
    pub fn new_with_hilo(deck: &mut Deck, init_hilo: i32) -> RoundState {
        let player_c1 = deal(deck);
        let player_c2 = deal(deck);
        let player = Hand::from(player_c1, player_c2);

        let dealer_card = deal(deck);
        let dealer = Hand::new().hit(dealer_card);

        let hilo = init_hilo + RoundState::card_hilo(dealer_card) +
            RoundState::card_hilo(player_c1) + RoundState::card_hilo(player_c2);

        RoundState { outcome: Outcome::Playing, player, dealer, hilo }
    }

    pub fn new(deck: &mut Deck) -> RoundState {
        RoundState::new_with_hilo(deck, 0)
    }

    pub fn hit(&self, deck: &mut Deck) -> Option<RoundState> {
        return match self.outcome {
            Playing => {
                let (new_player_hand, new_hilo) = RoundState::hit_player(&self.player, deck, self.hilo);
                return if new_player_hand.is_bust() {
                    Some(RoundState { outcome: Outcome::Lost, player: new_player_hand, dealer: self.dealer, hilo: new_hilo })
                } else {
                    Some(RoundState { outcome: Outcome::Playing, player: new_player_hand, dealer: self.dealer, hilo: new_hilo })
                };
            }

            _ => None
        };
    }

    pub fn stand(&self, deck: &mut Deck) -> Option<RoundState> {
        return match self.outcome {
            Playing => {
                let (dealer, new_hilo) = RoundState::hit_dealer(&self.dealer, deck, self.hilo);
                return if self.player > dealer {
                    Some(RoundState { outcome: Outcome::Won, player: self.player, dealer, hilo: new_hilo })
                } else if self.player == dealer {
                    Some(RoundState { outcome: Outcome::Draw, player: self.player, dealer, hilo: new_hilo })
                } else {
                    Some(RoundState { outcome: Outcome::Lost, player: self.player, dealer, hilo: new_hilo })
                };
            }

            _ => None
        };
    }

    pub fn won(&self) -> bool {
        return match self.outcome {
            Won => true,
            _ => false
        };
    }

    pub fn lost(&self) -> bool {
        return match self.outcome {
            Lost => true,
            _ => false
        };
    }

    pub fn draw(&self) -> bool {
        return match self.outcome {
            Draw => true,
            _ => false
        };
    }

    pub fn finished(&self) -> bool {
        return match self.outcome {
            Playing => false,
            _ => true
        };
    }

    fn card_hilo(card: u8) -> i32 {
        return if card >= 2 && card <= 6 {
            1
        } else if card >= 7 && card <= 9 {
            0
        } else {
            -1
        };
    }

    fn hit_player(player_hand: &Hand, deck: &mut Deck, hilo_acc: i32) -> (Hand, i32) {
        let card = deal(deck);
        (player_hand.hit(card), hilo_acc + RoundState::card_hilo(card))
    }

    fn hit_dealer(dealer_hand: &Hand, deck: &mut Deck, hilo_acc: i32) -> (Hand, i32) {
        let card = deal(deck);
        //      println!("Card dealt to dealer: {:?}", card);
        let new_card_hilo_acc = hilo_acc + RoundState::card_hilo(card);
        let new_dealer_hand = dealer_hand.hit(card);

        return if new_dealer_hand.sum < 17 {
            //        println!("Dealer sum {:?}, still less than 17", new_dealer_hand.sum);
            RoundState::hit_dealer(&new_dealer_hand, deck, new_card_hilo_acc)
        } else {
            //      println!("Dealer stays at sum {:?}", new_dealer_hand.sum);
            (new_dealer_hand, new_card_hilo_acc)
        };
    }
}

/// Assumes the deck has cards
fn deal(deck: &mut Deck) -> u8 {
    return deck.deal().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::deck::Deck;
    use super::*;

    #[test]
    fn test_losing_round() {
        let cards: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut deck = Deck::new_rigged(&cards);

        let start = RoundState::new(&mut deck);
        println!("Player got: {:?}", start.player);
        println!("Dealer got: {:?}", start.dealer);

        assert!(!start.finished());
        assert!(!start.lost());
        assert!(!start.won());
        assert!(!start.draw());

        let after_hit = start.hit(&mut deck).unwrap();
        println!("Player sum is: {:?}", after_hit.player);

        assert!(!after_hit.finished());
        assert!(!after_hit.lost());
        assert!(!after_hit.won());
        assert!(!after_hit.draw());

        let after_stand = after_hit.stand(&mut deck).unwrap();
        println!("Round finished: {:?}", after_stand);

        assert!(after_stand.finished());
        assert!(after_stand.lost());
        assert!(!after_stand.won());
        assert!(!after_stand.draw());
    }

    #[test]
    fn test_bust_round() {
        let cards: [u8; 10] = [10, 2, 3, 6, 9, 6, 7, 8, 9, 10];
        let mut deck = Deck::new_rigged(&cards);

        let start = RoundState::new(&mut deck);
        println!("Player got: {:?}", start.player);
        println!("Dealer got: {:?}", start.dealer);

        assert!(!start.finished());
        assert!(!start.lost());
        assert!(!start.won());
        assert!(!start.draw());

        let after_hit = start.hit(&mut deck).unwrap();
        println!("Player sum is: {:?}", after_hit.player);

        assert!(!after_hit.finished());
        assert!(!after_hit.lost());
        assert!(!after_hit.won());
        assert!(!after_hit.draw());

        let after_hit2 = after_hit.hit(&mut deck).unwrap();
        println!("Player sum is: {:?}", after_hit2.player);

        assert!(after_hit2.player.is_bust());
        assert!(after_hit2.finished());
        assert!(after_hit2.lost());
        assert!(!after_hit2.won());
        assert!(!after_hit2.draw());

        println!("Round finished: {:?}", after_hit2);
    }

    #[test]
    fn test_draw_round() {
        let cards: [u8; 10] = [10, 2, 2, 6, 10, 6, 7, 8, 9, 10];
        let mut deck = Deck::new_rigged(&cards);

        let start = RoundState::new(&mut deck);
        println!("Player got: {:?}", start.player);
        println!("Dealer got: {:?}", start.dealer);

        assert!(!start.finished());
        assert!(!start.lost());
        assert!(!start.won());
        assert!(!start.draw());

        let after_hit = start.hit(&mut deck).unwrap();
        println!("Player sum is: {:?}", after_hit.player);

        assert!(!after_hit.finished());
        assert!(!after_hit.lost());
        assert!(!after_hit.won());
        assert!(!after_hit.draw());

        let after_stand = after_hit.stand(&mut deck).unwrap();
        println!("Round finished: {:?}", after_stand);

        assert!(after_stand.finished());
        assert!(!after_stand.lost());
        assert!(!after_stand.won());
        assert!(after_stand.draw());
    }

    #[test]
    fn test_dealer_bust_round() {
        let cards: [u8; 10] = [10, 2, 2, 6, 10, 3, 7, 8, 9, 10];
        let mut deck = Deck::new_rigged(&cards);

        let start = RoundState::new(&mut deck);
        println!("Player got: {:?}", start.player);
        println!("Dealer got: {:?}", start.dealer);

        assert!(!start.finished());
        assert!(!start.lost());
        assert!(!start.won());
        assert!(!start.draw());

        let after_hit = start.hit(&mut deck).unwrap();
        println!("Player sum is: {:?}", after_hit.player);

        assert!(!after_hit.finished());
        assert!(!after_hit.lost());
        assert!(!after_hit.won());
        assert!(!after_hit.draw());

        let after_stand = after_hit.stand(&mut deck).unwrap();
        println!("Round finished: {:?}", after_stand);

        assert!(after_stand.dealer.is_bust());
        assert!(after_stand.finished());
        assert!(!after_stand.lost());
        assert!(after_stand.won());
        assert!(!after_stand.draw());
    }

    #[test]
    fn test_win_round() {
        let cards: [u8; 10] = [10, 3, 2, 6, 10, 6, 7, 8, 9, 10];
        let mut deck = Deck::new_rigged(&cards);

        let start = RoundState::new(&mut deck);
        println!("Player got: {:?}", start.player);
        println!("Dealer got: {:?}", start.dealer);

        assert!(!start.finished());
        assert!(!start.lost());
        assert!(!start.won());
        assert!(!start.draw());

        let after_hit = start.hit(&mut deck).unwrap();
        println!("Player sum is: {:?}", after_hit.player);

        assert!(!after_hit.finished());
        assert!(!after_hit.lost());
        assert!(!after_hit.won());
        assert!(!after_hit.draw());

        let after_stand = after_hit.stand(&mut deck).unwrap();
        println!("Round finished: {:?}", after_stand);

        assert!(after_stand.finished());
        assert!(!after_stand.lost());
        assert!(after_stand.won());
        assert!(!after_stand.draw());
    }

    #[test]
    fn test_card_hilo() {
        assert_eq!(RoundState::card_hilo(2), 1);
        assert_eq!(RoundState::card_hilo(3), 1);
        assert_eq!(RoundState::card_hilo(4), 1);
        assert_eq!(RoundState::card_hilo(5), 1);
        assert_eq!(RoundState::card_hilo(6), 1);
        assert_eq!(RoundState::card_hilo(7), 0);
        assert_eq!(RoundState::card_hilo(8), 0);
        assert_eq!(RoundState::card_hilo(9), 0);
        assert_eq!(RoundState::card_hilo(10), -1);
        assert_eq!(RoundState::card_hilo(1), -1);
    }

    #[test]
    fn test_hilo_counting_level() {
        let cards: [u8; 10] = [10, 9, 6, 2, 10, 6, 7, 8, 9, 10];
        let mut deck = Deck::new_rigged(&cards);

        let start = RoundState::new(&mut deck);
        println!("Player got: {:?}", start.player);
        println!("Dealer got: {:?}", start.dealer);

        assert_eq!(start.hilo, 0);
    }

    #[test]
    fn test_hilo_counting_lo() {
        let cards: [u8; 10] = [10, 10, 10, 1, 10, 1, 10, 10, 8, 9];
        let mut deck = Deck::new_rigged(&cards);

        let start = RoundState::new(&mut deck);
        println!("Player got: {:?}", start.player);
        println!("Dealer got: {:?}", start.dealer);

        assert_eq!(start.hilo, -3);

        let after_hit = start.hit(&mut deck).unwrap();
        println!("After hit: {:?}", after_hit);
        assert_eq!(after_hit.hilo, -4);

        let after_stand = after_hit.stand(&mut deck).unwrap();
        println!("End of round: {:?}", after_stand);
        assert_eq!(after_stand.hilo, -5);
    }

    #[test]
    fn test_hilo_counting_hi() {
        let cards: [u8; 10] = [2, 2, 3, 4, 5, 6, 5, 4, 3, 2];
        let mut deck = Deck::new_rigged(&cards);

        let start = RoundState::new(&mut deck);
        println!("Player got: {:?}", start.player);
        println!("Dealer got: {:?}", start.dealer);

        assert_eq!(start.hilo, 3);

        let after_hit = start.hit(&mut deck).unwrap();
        println!("After hit: {:?}", after_hit);
        assert_eq!(after_hit.hilo, 4);

        let after_stand = after_hit.stand(&mut deck).unwrap();
        println!("End of round: {:?}", after_stand);
        assert_eq!(after_stand.hilo, 7);
    }

    #[test]
    fn test_hilo_counting_0() {
        let cards: [u8; 10] = [7, 7, 7, 7, 7, 8, 8, 7, 7, 9];
        let mut deck = Deck::new_rigged(&cards);

        let start = RoundState::new(&mut deck);
        println!("Player got: {:?}", start.player);
        println!("Dealer got: {:?}", start.dealer);

        assert_eq!(start.hilo, 0);

        let after_hit = start.hit(&mut deck).unwrap();
        println!("After hit: {:?}", after_hit);
        assert_eq!(after_hit.hilo, 0);

        let after_stand = after_hit.stand(&mut deck).unwrap();
        println!("End of round: {:?}", after_stand);
        assert_eq!(after_stand.hilo, 0);
    }
}










