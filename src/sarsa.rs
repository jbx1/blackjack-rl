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

pub fn sarsa() {
    let mut q_table: QTable<BlackjackState, BlackjackAction> = QTable::new(0.0);

    let mut wins = 0;
    let mut losses = 0;
    let mut draws = 0;
    let mut avg_error = 0.0;
    let episodes = 500000;
    let mut count = 0.0;
    for i in 0..episodes {
        if i % 1000 == 0 {
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

//    let mean_error = sum_error / state_action_count as f64;
    (result.reward, 0.0)
}


pub struct EpisodeResult {
    state_actions: VecDeque<StateAction<BlackjackState, BlackjackAction>>,
    reward: i32,
}

pub fn episode(deck: &mut Deck, q_table: &mut QTable<BlackjackState, BlackjackAction>, episode_number: usize) -> EpisodeResult {
//    let step_size = 0.01;
    let mut state_actions: VecDeque<StateAction<BlackjackState, BlackjackAction>> = VecDeque::new();

    let mut round_state = RoundState::new(deck);
    let mut agent_state = BlackjackState::from(&round_state);
    let mut action = e_greedy_policy(&agent_state, q_table, episode_number);
    let mut state_action = StateAction{ agent_state, action };

    while !round_state.finished() {
        //apply the action
        state_actions.push_front(state_action);
        let new_round_state = match action {
            BlackjackAction::Hit => round_state.hit(deck).unwrap(),
            BlackjackAction::Stand => round_state.stand(deck).unwrap()
        };

        if round_state.player.sum >= 12 && round_state.player.sum <= 20 {
            let reward = if new_round_state.won() {
                1
            }
            else if new_round_state.lost() {
                -1
            } else {
                0
            } as f64;

            let q_next = if !new_round_state.finished() {
                //one-step look ahead
                let next_agent_state = BlackjackState::from(&new_round_state);

                let next_action = e_greedy_policy(&next_agent_state, q_table, episode_number);
                let next_state_action = StateAction{agent_state: next_agent_state, action: next_action };

                action = next_action;
                agent_state = next_agent_state;
                // q_table.get_value(&next_state_action)

                //for q-learning use greedy_policy() instead
                let best_action = greedy_policy(&agent_state, q_table, episode_number);
                q_table.get_value(&StateAction{agent_state: next_agent_state, action: best_action })
            } else {
                //q for terminal state is 0
                0.0
            };

            let count = q_table.get_count(&state_action);
            let q_value = q_table.get_value(&state_action);
            let step_size = 1.0 / (count + 1) as f64;
            let new_q_value = q_value + step_size * (reward + q_next - q_value);
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

pub fn greedy_policy(agent_state: &BlackjackState, q_table: &QTable<BlackjackState, BlackjackAction>, episode_number: usize) -> BlackjackAction {
    return if agent_state.player < 12 {
        BlackjackAction::Hit
    } else if agent_state.player == 21 {
        BlackjackAction::Stand
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
//    assuming the first episode is 0
//     let epsilon = if episode < 100000 {
//         //explore consistently for the first 100k episodes
//         0.1
//     } else {
//         //become greedier after 500k episodes
//         1.0 / (episode + 1) as f64
//     };

    let epsilon = 0.1;

//    let epsilon = 1.0 / (episode + 1) as f64;

//   let epsilon = f64::exp( episode as f64 / -10000.0);
//   println!("Epsilon: {:?}", epsilon);

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




