use std::collections::VecDeque;

use crate::blackjack_agent::{BlackjackAction, BlackjackState, EpisodeResult};
use crate::blackjack_policy::{e_greedy_policy, random_policy};
use crate::deck::Deck;
use crate::qtable::{QTable, StateAction};
use crate::round::RoundState;
use crate::trainer::Trainer;

pub fn monte_carlo() {
    println!("Running in Monte Carlo mode");
    let mut trainer = Trainer::new();
    trainer.train(evaluate_episode);
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
            q_table.update_value(&state_action, new_value);
        }
    }

    let mean_error = if state_action_count == 0 {
        0.0
    } else {
        sum_error / state_action_count as f64
    };

    (result.reward, mean_error)
}


pub fn episode(deck: &mut Deck, q_table: &QTable<BlackjackState, BlackjackAction>, episode_number: usize) -> EpisodeResult {
    let mut state_actions: VecDeque<StateAction<BlackjackState, BlackjackAction>> = VecDeque::new();

    let mut round_state = RoundState::new(deck);

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

    return EpisodeResult::from(&round_state, state_actions);
}






