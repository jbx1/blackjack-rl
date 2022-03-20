use std::collections::{HashMap, VecDeque};
use rand::{Rng, thread_rng};
use crate::deck::Deck;
use crate::round::{RoundState};

#[derive(Debug, Copy, Clone, Hash, Eq)]
pub struct AgentState {
    player: u8,
    ace: bool,
    dealer: u8
}

impl PartialEq for AgentState {
    fn eq(&self, other: &Self) -> bool {
        return self.ace == other.ace && self.player == other.player && self.dealer == other.dealer;
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Action {
    Hit,
    Stand
}

#[derive(Debug, Copy, Clone, Hash, Eq)]
pub struct StateAction {
    agent_state: AgentState,
    action: Action
}

impl PartialEq for StateAction {
    fn eq(&self, other: &Self) -> bool {
        return self.agent_state == other.agent_state && self.action == other.action;
    }
}


impl AgentState {
    pub fn from(round_state: &RoundState) -> AgentState {
        return AgentState{player: round_state.player.sum, ace: round_state.player.ace, dealer: round_state.dealer.sum};
    }
}

pub fn monte_carlo() {
    let mut q_values : HashMap<StateAction, f32> = HashMap::new();
    let mut counts : HashMap<StateAction, usize> = HashMap::new();

    for i in 1 .. 100000 {
        println!("Running episode {:?}", i);
        run_episode(&mut q_values, &mut counts);
    }

    let mut q : Vec<_> = q_values.iter().collect();
    q.sort_by(|a,b| b.1.partial_cmp(a.1).unwrap());

    for value in q {
        println!("{:?}", value);
    }
}

pub fn run_episode(q_values: &mut HashMap<StateAction, f32>, counts : &mut HashMap<StateAction, usize>) {
    let mut deck = Deck::new_shuffled();
    let result = episode(&mut deck);

    let default_q = 0.0;

    for state_action in result.state_actions {
        let old_value = q_values.get(&state_action).cloned().unwrap_or(default_q);
        let count = counts.get(&state_action).cloned().unwrap_or_default();

        let g = result.reward as f32;

        let new_count = count + 1;
        let new_value = old_value + (g - old_value) / (new_count as f32);

        counts.insert(state_action, new_count);
        q_values.insert(state_action, new_value);
    }
}

pub struct EpisodeResult {
    state_actions: VecDeque<StateAction>,
    reward: i32
}

pub fn episode(deck: &mut Deck) -> EpisodeResult  {

    let mut state_actions: VecDeque<StateAction> = VecDeque::new();

    let mut round_state = RoundState::new(deck);

    while !round_state.finished() {
        let mut agent_state = AgentState::from(&round_state);
        let mut action = naive_policy(&agent_state);

        //we push them to the front so that the last state-action pair are at the front
        state_actions.push_front(StateAction{ agent_state, action });

        match action {
            Action::Hit => round_state = round_state.hit(deck).unwrap(),
            Action::Stand => round_state = round_state.stand(deck).unwrap()
        }
    }

    return if round_state.won() {
        EpisodeResult { state_actions, reward: 1 }
    } else if round_state.lost() {
        EpisodeResult { state_actions, reward: -1 }
    } else {
        EpisodeResult { state_actions, reward: 0 }
    }
}

pub fn random_policy(agent_state: &AgentState) -> Action {
    return if agent_state.player < 12 {
        Action::Hit
    } else if agent_state.player >= 20 {
        Action::Stand
    }
    else {
        match thread_rng().gen_range(0..2) {
            0 => Action::Hit,
            _ => Action::Stand
        }
    }
}

pub fn naive_policy(agent_state: &AgentState) -> Action {
    return if agent_state.player < 20 {
        Action::Hit
    } else {
        Action::Stand
    }
}




