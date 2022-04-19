use rand::{Rng, thread_rng};
use crate::blackjack_agent::{BlackjackAction, BlackjackState};
use crate::qtable::QTable;

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

pub fn greedy_policy(agent_state: &BlackjackState, q_table: &QTable<BlackjackState, BlackjackAction>, _episode_number: usize) -> BlackjackAction {
    return if agent_state.player < 12 {
        BlackjackAction::Hit
    } else if agent_state.player == 21 {
        BlackjackAction::Stand
    } else {
        q_table.select_greedy_action(agent_state).unwrap_or_else(|| random_action())
    };
}

pub fn random_policy(agent_state: &BlackjackState, _q_table: &QTable<BlackjackState, BlackjackAction>, _episode_number: usize) -> BlackjackAction {
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

    // let episode = 0.1;
    // let epsilon = 1.0 / (episode + 1) as f64;
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