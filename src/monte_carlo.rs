use std::collections::{VecDeque};
use rand::{Rng, thread_rng};
use crate::deck::Deck;
use crate::qtable::{Action, QTable, State, StateAction};
use crate::round::{RoundState};

#[derive(Debug, Copy, Clone, Hash, Eq)]
pub struct BlackjackState {
    player: u8,
    dealer: u8,
    ace: bool,
}

impl State for BlackjackState {}

impl PartialEq for BlackjackState {
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

impl BlackjackState {
    pub fn from(round_state: &RoundState) -> BlackjackState {
        return BlackjackState { player: round_state.player.sum, ace: round_state.player.ace, dealer: round_state.dealer.sum };
    }
}

pub fn monte_carlo() {
    let mut q_table: QTable<BlackjackState, BlackjackAction> = QTable::new(0.0);

    let mut wins = 0;
    let mut losses = 0;
    let mut draws = 0;
    let mut avg_error = 0.0;
    let episodes = 500000;
    let mut count = 0.0;
    for i in 0..episodes {
        if i % 1000 == 0
            {
            println!("\"{:?}\",\"{:?}\",\"{:?}\",\"{:?}\",\"{:?}\"", i, wins, losses, draws, avg_error);
            wins = 0;
            losses = 0;
            draws = 0;
        }

        count = count + 1.0;
        let (reward, error) = evaluate_episode(&mut q_table, i);
        if reward > 0 {
            wins += 1;
        } else if reward < 0 {
            losses += 1;
        } else {
            draws += 1;
        }

        avg_error = avg_error + (error - avg_error) / count;
    }

    println!("\"{:?}\",\"{:?}\",\"{:?}\",\"{:?}\",\"{:?}\"", episodes-1, wins, losses, draws, avg_error);

    let q_values = q_table.get_all_values();
    println!("Total state action values: {:?}", q_values.len());
    for value in q_values {
        println!("{:?}", value);
    }
}

pub fn evaluate_episode(q_table: &mut QTable<BlackjackState, BlackjackAction>, episode_number: usize) -> (i32, f64) {
    let mut deck = Deck::new_shuffled();
    let result = episode(&mut deck, q_table, episode_number);

    let mut sum_error = 0.0;
    let state_action_count = result.state_actions.len();
    for state_action in result.state_actions {
        if state_action.agent_state.player > 11 && state_action.agent_state.player < 21 {
            let old_value = q_table.get_value(&state_action);
            let count = q_table.get_count(&state_action) + 1;

            let g = result.reward as f64;

            let error = g - old_value;
            let new_value = old_value + (error / count as f64);

            sum_error = sum_error + f64::abs(error);

    //        println!("Old value for {:?} was {:?} while reward {:?} error is {:?}, new q value is {:?}, count is {:?}", state_action, old_value, g, error, new_value, count);

            q_table.update_value(&state_action, new_value);
        }
    }

    let mean_error = sum_error / state_action_count as f64;
    (result.reward, mean_error)
}


pub struct EpisodeResult {
    state_actions: VecDeque<StateAction<BlackjackState, BlackjackAction>>,
    reward: i32,
}

pub fn episode(deck: &mut Deck, q_table: &QTable<BlackjackState, BlackjackAction>, episode_number: usize) -> EpisodeResult {
    let mut state_actions: VecDeque<StateAction<BlackjackState, BlackjackAction>> = VecDeque::new();

    let mut round_state = RoundState::new(deck);

//    let mut random_start = episode_number < 100000;
    let mut random_start = false;

    while !round_state.finished() {
        let agent_state = BlackjackState::from(&round_state);
        let action = if random_start {
            random_start = false;
            random_policy(&agent_state, q_table, episode_number)
        } else {
            e_greedy_policy(&agent_state, q_table, episode_number)
        };

        //we push them to the front so that the last state-action pair are at the front
        state_actions.push_front(StateAction { agent_state, action });

        match action {
            BlackjackAction::Hit => round_state = round_state.hit(deck).unwrap(),
            BlackjackAction::Stand => round_state = round_state.stand(deck).unwrap()
        }
    }

    return if round_state.won() {
        EpisodeResult { state_actions, reward: 1 }
    } else if round_state.lost() {
        EpisodeResult { state_actions, reward: -1 }
    } else {
        EpisodeResult { state_actions, reward: 0 }
    };
}

pub fn e_greedy_policy(agent_state: &BlackjackState, q_table: &QTable<BlackjackState, BlackjackAction>, episode_number: usize) -> BlackjackAction {
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

pub fn random_policy(agent_state: &BlackjackState, q_table: &QTable<BlackjackState, BlackjackAction>, episode_number: usize) -> BlackjackAction {
    return if agent_state.player < 12 {
        BlackjackAction::Hit
    } else if agent_state.player == 21 {
        BlackjackAction::Stand
    } else {
        random_action()
    }
}

fn epsilon_explore(episode: usize) -> bool {
    //assuming the first episode is 0
//    let epsilon = 1.0 / (episode + 1) as f64;

    let epsilon = f64::exp( episode as f64 / -10000.0);
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




