use crate::network::{ Network };
use crate::premise::Premise;
use crate::context::Context;
use std::collections::HashSet;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt;
use std::fmt::Debug;


pub enum Action {
    AmbChoose
}

#[derive(Clone)]
pub struct TruthManagementContext<T> {
    premise_outness: HashSet<T>,
    premise_nogoods: HashMap<T, Vec<T>>,
}

impl<T> fmt::Debug for TruthManagementContext<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Truth Management System")

        //TODO premises
    }
}

impl<T: Premise> TruthManagementContext<T> {
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
}

impl<T: Premise> Context for TruthManagementContext<T> {
    type Action = Action;

    fn run_action(&mut self, action: Self::Action) {
        match action {
            Action::AmbChoose => {
            }
        }
    }

}
