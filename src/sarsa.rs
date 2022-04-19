use std::collections::{VecDeque};
use crate::blackjack_agent::{BlackjackAction, BlackjackState, EpisodeResult, reward};
use crate::blackjack_policy::{e_greedy_policy, greedy_policy};
use crate::deck::Deck;
use crate::qtable::{QTable, StateAction};
use crate::round::{RoundState};
use crate::trainer::Trainer;

pub enum Mode {
    SARSA,
    SARSAMAX //a.k.a Q-Learning
}

pub fn sarsa() {
  println!("Running in SARSA mode");
  let mut trainer = Trainer::new();
  trainer.train(evaluate_episode_sarsa);
}

pub fn sarsamax() {
    println!("Running in SARSAMAX (Q-Learning) mode");
    let mut trainer = Trainer::new();
    trainer.train(evaluate_episode_sarsamax);
}

pub fn evaluate_episode_sarsa(q_table: &mut QTable<BlackjackState, BlackjackAction>, episode_number: usize) -> (i32, f64) {
    let mut deck = Deck::new_shuffled();
    let (result, error) = episode(&mut deck, q_table, episode_number, Mode::SARSA);
    (result.reward, error)
}

pub fn evaluate_episode_sarsamax(q_table: &mut QTable<BlackjackState, BlackjackAction>, episode_number: usize) -> (i32, f64) {
    let mut deck = Deck::new_shuffled();
    let (result, error) = episode(&mut deck, q_table, episode_number, Mode::SARSAMAX);
    (result.reward, error)
}

pub fn episode(deck: &mut Deck, q_table: &mut QTable<BlackjackState, BlackjackAction>, episode_number: usize, mode: Mode ) -> (EpisodeResult, f64) {

    let mut state_actions: VecDeque<StateAction<BlackjackState, BlackjackAction>> = VecDeque::new();

    let mut round_state = RoundState::new(deck);
    let mut agent_state = BlackjackState::from(&round_state);
    let mut action = e_greedy_policy(&agent_state, q_table, episode_number);
    let mut state_action = StateAction{ agent_state, action };
    let mut sum_error = 0.0;
    let mut state_action_count = 0;

    while !round_state.finished() {
        //apply the action
        state_actions.push_front(state_action);
        let new_round_state = match action {
            BlackjackAction::Hit => round_state.hit(deck).unwrap(),
            BlackjackAction::Stand => round_state.stand(deck).unwrap()
        };

        if round_state.player.sum >= 12 && round_state.player.sum <= 20 {
            //todo: this could do with some refactoring!
            let reward = reward(&new_round_state) as f64;

            let q_next = if !new_round_state.finished() {
                //choose the next action according to the policy
                let new_agent_state = BlackjackState::from(&new_round_state);
                let next_action = e_greedy_policy(&new_agent_state, q_table, episode_number);
                let next_state_action = StateAction{agent_state: new_agent_state, action: next_action };

                let q = match mode {
                    Mode::SARSA => {
                        q_table.get_value(&next_state_action)
                    }

                    Mode::SARSAMAX => {
                        //a.k.a q-learning
                        let best_action = greedy_policy(&agent_state, q_table, episode_number);
                        q_table.get_value(&StateAction{agent_state: new_agent_state, action: best_action })
                    }
                };

                action = next_action;
                agent_state = new_agent_state;
                q
            } else {
                //q for terminal state is 0
                0.0
            };

            let count = q_table.get_count(&state_action);
            let q_value = q_table.get_value(&state_action);
            let step_size = 1.0 / (count + 1) as f64;

            let error = reward + q_next - q_value;
            sum_error = sum_error + f64::abs(error);
            state_action_count += 1;

            let new_q_value = q_value + (step_size * error);
            if new_q_value != q_value {

                q_table.update_value(&state_action, new_q_value);
            }

        } else if !round_state.finished() {
            //we had an obvious action no need to update q_values

            //compute the new state, and find what the next action should be
            agent_state = BlackjackState::from(&new_round_state);
            action = e_greedy_policy(&agent_state, q_table, episode_number);
        }

        state_action = StateAction{agent_state, action};
        round_state = new_round_state;
    }


    let mean_error = if state_action_count == 0 {
        0.0
    } else {
        sum_error / state_action_count as f64
    };

    return (EpisodeResult::from(&round_state, state_actions), mean_error);
}












