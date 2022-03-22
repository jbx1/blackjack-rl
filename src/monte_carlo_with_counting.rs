use std::collections::{VecDeque};
use rand::{Rng, thread_rng};
use crate::deck::Deck;
use crate::qtable::{Action, QTable, State, StateAction};
use crate::round::{RoundState};

#[derive(Debug, Copy, Clone, Hash, Eq)]
pub struct BlackjackCountingState {
    player: u8,
    dealer: u8,
    ace: bool
}

impl State for BlackjackCountingState {}

impl PartialEq for BlackjackCountingState {
    fn eq(&self, other: &Self) -> bool {
        return self.ace == other.ace && self.player == other.player && self.dealer == other.dealer;
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BlackjackAction {
    Hit,
    Stand,
}

impl Action for BlackjackAction {}

impl BlackjackCountingState {
    pub fn from(round_state: &RoundState) -> BlackjackCountingState {
        return BlackjackCountingState { player: round_state.player.sum, ace: round_state.player.ace, dealer: round_state.dealer.sum};
    }
}

/// experimental function with card counting
pub fn monte_carlo_cardcounting() {
    let mut q_table: QTable<BlackjackCountingState, BlackjackAction> = QTable::new(0.0);

    let mut wins = 0;
    let mut losses = 0;
    let mut draws = 0;

    let mut money = 0;

    let episodes = 6000000;
    let mut deck = Deck::new_shuffled();
    let mut current_hilo = 0;

    for i in 0..episodes {
        if i % 1000 == 0 {
            println!("\"{:?}\",\"{:?}\",\"{:?}\",\"{:?}\",\"{:?}\"", i, wins, losses, draws, money);
            wins = 0;
            losses = 0;
            draws = 0;
            money = 0;
        }

        if deck.len() < 15 {
            deck = Deck::new_shuffled();
            current_hilo = 0;
        }

        let (reward, new_hilo) = evaluate_episode(&mut q_table, i, &mut deck, current_hilo);
        money += if current_hilo > 1 {
            reward * current_hilo
        } else {
            reward
        };

        current_hilo = new_hilo;

        if reward > 0 {
            wins += 1
        } else if reward < 0 {
            losses += 1
        } else {
            draws += 1
        }
    }

    println!("\"{:?}\",\"{:?}\",\"{:?}\",\"{:?}\",\"{:?}\"", episodes, wins, losses, draws, money);

    println!("Finished - Wins: {:?} Losses: {:?} Draws {:?}", wins, losses, draws);

    let q_values = q_table.get_all_values();
    println!("Total state action values: {:?}", q_values.len());
    for value in q_values {
        println!("{:?}", value);
    }
}

pub fn evaluate_episode(q_table: &mut QTable<BlackjackCountingState, BlackjackAction>, episode_number: usize, deck: &mut Deck, init_hilo: i32) -> (i32, i32) {
    let result = episode(deck, q_table, episode_number, init_hilo);

    let mut largest_error = 0.0;

    for state_action in result.state_actions {
        if state_action.agent_state.player > 11 && state_action.agent_state.player < 21 {
            let old_value = q_table.get_value(&state_action);
            let count = q_table.get_count(&state_action) + 1;

            let g = result.reward as f64;

            let error = g - old_value;
            largest_error = f64::max(largest_error, f64::abs(error));
            let new_value = old_value + (error / count as f64);
    //        println!("Old value for {:?} was {:?} while reward {:?} error is {:?}, new q value is {:?}, count is {:?}", state_action, old_value, g, error, new_value, count);

            q_table.update_value(&state_action, new_value);
        }
    }

    (result.reward, result.hilo)
}


pub struct EpisodeResult {
    state_actions: VecDeque<StateAction<BlackjackCountingState, BlackjackAction>>,
    reward: i32,
    hilo: i32
}

pub fn episode(deck: &mut Deck, q_table: &QTable<BlackjackCountingState, BlackjackAction>, episode_number: usize, init_hilo : i32) -> EpisodeResult {
    let mut state_actions: VecDeque<StateAction<BlackjackCountingState, BlackjackAction>> = VecDeque::new();

    let mut round_state = RoundState::new_with_hilo(deck, init_hilo);

    while !round_state.finished() {
        let agent_state = BlackjackCountingState::from(&round_state);
        let action = e_greedy_policy(&agent_state, q_table, episode_number);

        //we push them to the front so that the last state-action pair are at the front
        state_actions.push_front(StateAction { agent_state, action });

        match action {
            BlackjackAction::Hit => round_state = round_state.hit(deck).unwrap(),
            BlackjackAction::Stand => round_state = round_state.stand(deck).unwrap()
        }
    }

    let reward = if round_state.won() {
        1
        // if init_hilo > 1 {
        //     2
        // } else {
        //     1
        // }
    } else if round_state.lost() {
        -1
        // if init_hilo > 1 {
        //     -2
        // } else {
        //     -1
        // }
    }
    else {
        0
    };

    EpisodeResult { state_actions, reward, hilo: round_state.hilo }
}

pub fn e_greedy_policy(agent_state: &BlackjackCountingState, q_table: &QTable<BlackjackCountingState, BlackjackAction>, episode_number: usize) -> BlackjackAction {
    return if agent_state.player < 12 {
        BlackjackAction::Hit
    } else if agent_state.player == 21 {
        BlackjackAction::Stand
    } else if epsilon_explore(episode_number) {
        random_action()
    } else {
        q_table.select_greedy_action(agent_state).unwrap_or_else(|| random_action())
    };
}

fn epsilon_explore(episode: usize) -> bool {
    //assuming the first episode is 0
    let epsilon = if episode < 1000000 {
        //explore consistently for the first 400k episodes
        0.1
    } else {
        //become greedier after 500k episodes
        1.0 / (episode + 1) as f64
    };

 //   let epsilon = 1.0 / (episode + 1) as f64;

    //this generates a number between 0 (inclusive) and 1 (exclusive)
    let rnd = thread_rng().gen::<f64>();

    return rnd < epsilon;
}

fn random_action() -> BlackjackAction {
    match thread_rng().gen_range(0..2) {
        0 => BlackjackAction::Hit,
        _ => BlackjackAction::Stand
    }
}




