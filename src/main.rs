use std::io::stdin;
use std::time::{Instant};
use blackjack::monte_carlo::monte_carlo;
use crate::deck::Deck;
use crate::round::{Outcome, RoundState};

pub mod round;
pub mod deck;
pub mod hand;

fn play() {
    println!("Welcome to Simple Blackjack");
    let mut deck = Deck::new_shuffled();
    let mut round = RoundState::new(&mut deck);

    println!("Cards are dealt: {:?}", round);

    while !round.finished() {
        println!("Current round state: {:?}", round);
        println!("Hit (h) or Stand (s)? ");
        let mut choice = String::new();
        stdin().read_line(&mut choice).unwrap();

        choice = choice.trim().to_lowercase();
        match choice.as_str() {
            "h" => round = round.hit(&mut deck).unwrap(),
            "s" => round = round.stand(&mut deck).unwrap(),
            _ => println!("Invalid option {:?}", choice)
        }
    }

    println!("Finished: {:?}", round);
    match round.outcome {
        Outcome::Won => println!("Congratulations! You won!"),
        Outcome::Lost => println!("Sorry! You lost."),
        Outcome::Draw => println!("It's a Draw!"),
        Outcome::Playing => println!("Invalid state, the game has not finished yet.")
    }
}

fn main() {

    //play();
    let start = Instant::now();
    monte_carlo();
    let dur = start.elapsed();
    println!("Total time: {:?}", dur);
}
