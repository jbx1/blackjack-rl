use std::collections::VecDeque;
use crate::qtable::{Action, State, StateAction};
use crate::round::{Outcome, RoundState};

#[derive(Debug, Copy, Clone, Hash, Eq)]
pub struct BlackjackState {
    pub player: u8,
    pub dealer: u8,
    pub ace: bool,
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

pub struct EpisodeResult {
    pub state_actions: VecDeque<StateAction<BlackjackState, BlackjackAction>>,
    pub reward: i32,
}

impl EpisodeResult {
    pub fn from(round_state: &RoundState, state_actions: VecDeque<StateAction<BlackjackState, BlackjackAction>>) -> EpisodeResult {
        return EpisodeResult { state_actions, reward: reward(round_state)}
    }
}

pub fn reward(round_state: &RoundState) -> i32 {
    match round_state.outcome {
        Outcome::Won => 1,
        Outcome::Lost => -1,
        Outcome::Draw => 0,
        Outcome::Playing => 0
    }
}