/// Borrowed from https://github.com/dszoboszlay/adventofcode/blob/main/rust/src/utils/astar.rs

use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::hash::Hash;

/// An interface for representing search states.
///
/// A search state needs to provide some basic information on whether it is a
/// goal state and its cost. It also has to provide an iterator for the search
/// states reachable from it.
///
/// Search states also have a key used for detecting loops.
pub trait SearchState {
    /// The type of the state's key.
    type Key: Hash + Ord + Clone;
    type Iter: Iterator<Item = Self>;

    fn key(&self) -> Self::Key;
    fn is_goal(&self) -> bool;
    fn cost(&self) -> usize;
    fn heuristic(&self) -> usize;
    fn next_states(self) -> Self::Iter;
}

type StatesKey<K> = (usize, K);

struct Search<S, K> {
    states: BTreeMap<StatesKey<K>, S>,
    best_costs: BTreeMap<K, usize>,
}

impl<S: SearchState> Search<S, S::Key> {
    fn new() -> Search<S, S::Key> {
        Search {
            states: BTreeMap::new(),
            best_costs: BTreeMap::new(),
        }
    }

    fn insert(&mut self, search_state: S) {
        let total = search_state.cost() + search_state.heuristic();
        match self.best_costs.entry(search_state.key()) {
            Entry::Vacant(entry) => {
                entry.insert(total);
                self.states
                    .insert((total, search_state.key()), search_state);
            }
            Entry::Occupied(mut entry) => {
                let old_total = *entry.get();
                if total < old_total {
                    let mut states_key = (old_total, search_state.key());
                    entry.insert(total);
                    self.states.remove(&states_key);
                    states_key.0 = total;
                    self.states.insert(states_key, search_state);
                }
            }
        }
    }

    fn pop(&mut self) -> Option<S> {
        match self.states.keys().next() {
            None => None,
            Some(key) => {
                let key_copy = (*key).clone();
                self.states.remove(&key_copy)
            }
        }
    }
}

pub fn solve<S: SearchState>(start_state: S) -> Option<S> {
    let mut search = Search::new();
    search.insert(start_state);
    while let Some(state) = search.pop() {
        if state.is_goal() {
            return Some(state);
        }

        state.next_states().for_each(|s| search.insert(s));
    }

    None
}
