use std::collections::{HashMap, VecDeque};
use rand::{Rng, thread_rng};
use crate::deck::Deck;
use crate::qtable::{Action, QTable, State};
use crate::round::{RoundState};

#[derive(Debug, Copy, Clone, Hash, Eq)]
pub struct BlackjackState {
    player: u8,
    ace: bool,
    dealer: u8,
}

impl State for BlackjackState {

}

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

impl Action for BlackjackAction {

}

#[derive(Debug, Copy, Clone, Hash, Eq)]
pub struct StateAction {
    agent_state: BlackjackState,
    action: BlackjackAction,
}

impl PartialEq for StateAction {
    fn eq(&self, other: &Self) -> bool {
        return self.agent_state == other.agent_state && self.action == other.action;
    }
}


impl BlackjackState {
    pub fn from(round_state: &RoundState) -> BlackjackState {
        return BlackjackState { player: round_state.player.sum, ace: round_state.player.ace, dealer: round_state.dealer.sum };
    }
}

pub fn monte_carlo() {

    // let mut q_table: QTable<BlackjackState, BlackjackAction> = QTable::new(0.0);

    let mut q_values: HashMap<BlackjackState, HashMap<BlackjackAction, f64>> = HashMap::new();
    let mut counts: HashMap<StateAction, usize> = HashMap::new();

    for i in 0..500000 {
        if i % 1000 == 0 {
            println!("Running episode {:?}", i);
        }
        evaluate_episode(&mut q_values, &mut counts, i);
    }

    let mut q: Vec<_> = q_values.iter()
        .map(|(k, l)|
                 l.iter().map(|(a, v)| (StateAction{agent_state: k.clone(), action: a.clone()}, v)))
        .flatten()
        .collect();

 //   let mut q: Vec<_> = q_values.iter().collect();
    q.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    println!("Total state action values: {:?}", q.len());
    for value in q {
        println!("{:?}", value);
    }
}

pub fn evaluate_episode(q_values: &mut HashMap<BlackjackState, HashMap<BlackjackAction, f64>>, counts: &mut HashMap<StateAction, usize>, episode_number : usize) -> f64 {
    let mut deck = Deck::new_shuffled();
    let result = episode(&mut deck, q_values, episode_number);

    let default_q = 0.0;
    let mut largest_error = 0.0;

    for state_action in result.state_actions {
        let old_value = get_state_action_value(&state_action, q_values, default_q);
        let count = counts.get(&state_action).cloned().unwrap_or_default();

        let g = result.reward as f64;

        let new_count = count + 1;
        let error = g - old_value;
        largest_error = f64::max(largest_error, error);
        let new_value = old_value + error / (new_count as f64);

        counts.insert(state_action, new_count);
        update_state_action_value(&state_action, q_values, new_value);
    }

    largest_error
}


pub struct EpisodeResult {
    state_actions: VecDeque<StateAction>,
    reward: i32,
}

pub fn episode(deck: &mut Deck, q_values: &HashMap<BlackjackState, HashMap<BlackjackAction, f64>>, episode_number: usize) -> EpisodeResult {
    let mut state_actions: VecDeque<StateAction> = VecDeque::new();

    let mut round_state = RoundState::new(deck);

    while !round_state.finished() {
        let agent_state = BlackjackState::from(&round_state);
        let action = e_greedy_policy(&agent_state, q_values, episode_number);

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

pub fn e_greedy_policy(agent_state: &BlackjackState, q_values: &HashMap<BlackjackState, HashMap<BlackjackAction, f64>>, episode_number: usize) -> BlackjackAction {
    return if agent_state.player < 12 {
        BlackjackAction::Hit
    } else if agent_state.player == 21 {
        BlackjackAction::Stand
    } else if epsilon_explore(episode_number) {
        random_action()
    } else {
        select_greedy_action(agent_state, q_values)
    };
}

fn get_state_action_value(state_action: &StateAction, q_values: &HashMap<BlackjackState, HashMap<BlackjackAction, f64>>, default: f64) -> f64 {
    q_values.get(&state_action.agent_state)
        .and_then(|map| map.get(&state_action.action))
        .map(|a| *a)
        .unwrap_or(default)
}

fn update_state_action_value(state_action: &StateAction, q_values: &mut HashMap<BlackjackState, HashMap<BlackjackAction, f64>>, new_value: f64) {
    let state_action_values = q_values.entry(state_action.agent_state).or_insert_with(|| HashMap::new());
    state_action_values.insert(state_action.action, new_value);
}

fn select_greedy_action(agent_state: &BlackjackState, q_values: &HashMap<BlackjackState, HashMap<BlackjackAction, f64>>) -> BlackjackAction {
    let action_values = q_values.get(agent_state);

    action_values
        .filter(|map| !map.is_empty())
        .and_then(|map| select_best_action(map))
        .map(|opt| opt.clone())
        .unwrap_or_else(|| random_action())
}

fn select_best_action(action_values: &HashMap<BlackjackAction, f64>) -> Option<&BlackjackAction> {
    let mut q: Vec<_> = action_values.iter().collect();
    q.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    q.first().map(|v| v.0)
}

fn epsilon_explore(episode: usize) -> bool {
    //assuming the first episode is 0
    let epsilon = 1.0 / (episode + 1) as f64;

    //this generates a number between 0 (inclusive) and 1 (exclusive)
    let rnd = thread_rng().gen::<f64>();

    return rnd < epsilon;
}

pub fn random_policy(agent_state: &BlackjackState) -> BlackjackAction {
    return if agent_state.player < 12 {
        BlackjackAction::Hit
    } else if agent_state.player >= 20 {
        BlackjackAction::Stand
    } else {
        random_action()
    };
}

fn random_action() -> BlackjackAction {
    match thread_rng().gen_range(0..2) {
        0 => BlackjackAction::Hit,
        _ => BlackjackAction::Stand
    }
}

pub fn naive_policy(agent_state: &BlackjackState) -> BlackjackAction {
    return if agent_state.player < 20 {
        BlackjackAction::Hit
    } else {
        BlackjackAction::Stand
    };
}




