use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;


pub trait State: Eq + Hash + Clone + Debug {}

pub trait Action: Eq + Hash + Clone + Debug {}

#[derive(Debug, Copy, Clone, Hash, Eq)]
pub struct StateAction<S: State, A: Action> {
    pub agent_state: S,
    pub action: A,
}

impl<S: State, A: Action> PartialEq for StateAction<S, A> {
    fn eq(&self, other: &Self) -> bool {
        return self.agent_state == other.agent_state && self.action == other.action;
    }
}

pub struct QTable<S: State, A: Action> {
    q_values: HashMap<S, HashMap<A, f64>>,
    counts: HashMap<StateAction<S, A>, usize>,
    default_value: f64,
}

impl<S: State, A: Action> QTable<S, A> {
    pub fn new(default_value: f64) -> QTable<S, A> {
        return QTable { q_values: HashMap::new(), counts: HashMap::new(), default_value };
    }

    pub fn get_value(&self, state_action: &StateAction<S, A>) -> f64 {
        self.q_values.get(&state_action.agent_state)
            .and_then(|map| map.get(&state_action.action))
            .map(|a| *a)
            .unwrap_or(self.default_value)
    }

    pub fn get_count(&self, state_action: &StateAction<S, A>) -> usize {
        self.counts
            .get(state_action)
            .cloned()
            .unwrap_or_default()
    }

    pub fn update_value(&mut self, state_action: &StateAction<S, A>, new_value: f64) {
        let state_action_values = self.q_values
            .entry(state_action.agent_state.clone())
            .or_insert_with(|| HashMap::new());

        state_action_values.insert(state_action.action.clone(), new_value);
        *self.counts.entry(state_action.clone()).or_insert_with(|| 0) += 1;
    }

    pub fn select_greedy_action(&self, agent_state: &S) -> Option<A> {
        let action_values: Option<&HashMap<A, f64>> = self.q_values.get(agent_state);

        action_values
            .filter(|map| !map.is_empty())
            .and_then(|map| self.select_best_action(map))
            .map(|opt| opt.clone())
    }

    fn select_best_action(&self, action_values: &HashMap<A, f64>) -> Option<A> {
        let mut q: Vec<_> = action_values.iter().collect();
        q.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

        q.first().map(|v| v.0.clone())
    }

    pub fn get_all_values(&self) -> Vec<(StateAction<S, A>, f64)> {
        // println!("Counts:");
        // for (key, value) in &self.counts {
        //     println!("{:?}: {:?}", key, value);
        // }

        let mut q: Vec<(StateAction<S, A>, f64)> = self.q_values.iter()
            .map(|(k, l)|
                l.iter().map(|(a, v)| (StateAction { agent_state: k.clone(), action: a.clone() }, v.clone())))
            .flatten()
            .collect();

        q.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        return q;
    }
}