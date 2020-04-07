use crate::network::{ Network };
use crate::premise::Premise;
use crate::context::Context;
use std::collections::HashSet;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt;
use std::fmt::Debug;


pub enum Event {
    Changed,
    Unchanged
}

#[derive(Clone)]
pub struct TruthManagementSystem<T> {
    premise_outness: HashSet<T>,
    premise_nogoods: HashMap<T, Vec<T>>,
}

impl<T: Clone + Debug> Context for TruthManagementSystem<T> {}

impl<T> fmt::Debug for TruthManagementSystem<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Truth Management System")

        //TODO premises
    }
}

impl<T: Premise> TruthManagementSystem<T> {
    pub fn new() -> Self {
        Self {
            premise_outness: HashSet::new(),
            premise_nogoods: HashMap::new()
        }
    }

    pub fn premise_in(&self, premise: &T) -> bool {
        !self.premise_outness.contains(premise)
    }

    pub fn reasons_against_premise(&self, premise: &T) -> Option<impl Iterator<Item=&T>> {
        self.premise_nogoods.get(premise).map(|vec| vec.iter())
    }

    pub fn kick_out_premise(&mut self, premise: T) -> Event {
        if self.premise_outness.contains(&premise) {
            self.premise_outness.remove(&premise);
            Event::Changed
        }
        else {
            Event::Unchanged
        }
    }

    pub fn bring_in_premise(&mut self, premise: T) -> Event {
        if !self.premise_outness.contains(&premise) {
            self.premise_outness.insert(premise);
            Event::Changed
        }
        else {
            Event::Unchanged
        }
    }
}
