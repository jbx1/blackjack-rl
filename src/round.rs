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
}

impl RoundState {
    pub fn new(deck: &mut Deck) -> RoundState {
        let player_c1 = deal(deck);
        let player_c2 = deal(deck);
  //      println!("Cards dealt to player: {:?} {:?}", player_c1, player_c2);

        let player = Hand::from(player_c1, player_c2);

        let dealer_card = deal(deck);
//        println!("Card dealt to dealer: {:?}", dealer_card);
        let dealer = Hand::new().hit(dealer_card);

        return RoundState { outcome: Outcome::Playing, player, dealer };
    }

    pub fn hit(&self, deck: &mut Deck) -> Option<RoundState> {
        return match self.outcome {
            Playing => {
                let new_player_hand = RoundState::hit_player(&self.player, deck);
                return if new_player_hand.is_bust() {
                    Some(RoundState { outcome: Outcome::Lost, player: new_player_hand, dealer: self.dealer })
                } else {
                    Some(RoundState { outcome: Outcome::Playing, player: new_player_hand, dealer: self.dealer })
                };
            }

            _ => None
        };
    }

    pub fn stand(&self, deck: &mut Deck) -> Option<RoundState> {
        return match self.outcome {
            Playing => {
                let dealer = RoundState::hit_dealer(&self.dealer, deck);
                return if self.player > dealer {
                    Some(RoundState { outcome: Outcome::Won, player: self.player, dealer })
                } else if self.player == dealer {
                    Some(RoundState { outcome: Outcome::Draw, player: self.player, dealer })
                } else {
                    Some(RoundState { outcome: Outcome::Lost, player: self.player, dealer })
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


    fn hit_player(player_hand: &Hand, deck: &mut Deck) -> Hand {
        let card = deal(deck);
//        println!("Card dealt to player: {:?}", card);
        return player_hand.hit(card);
    }

    fn hit_dealer(dealer_hand: &Hand, deck: &mut Deck) -> Hand {
        let card = deal(deck);
  //      println!("Card dealt to dealer: {:?}", card);
        let new_dealer_hand = dealer_hand.hit(card);

        return if new_dealer_hand.sum < 17 {
    //        println!("Dealer sum {:?}, still less than 17", new_dealer_hand.sum);
            RoundState::hit_dealer(&new_dealer_hand, deck)
        } else {
      //      println!("Dealer stays at sum {:?}", new_dealer_hand.sum);
            new_dealer_hand
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

}









