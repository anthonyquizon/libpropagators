use crate::network::{ Network };
use std::collections::HashSet;
use std::hash::{Hash};
use std::rc::{Rc};
use std::cell::RefCell;
use std::fmt::Debug;


pub trait Premise: Clone + Hash + Ord + PartialOrd + PartialEq + Eq + Debug {}

impl Premise for String {}

#[derive(Clone)]
pub struct TruthManagementSystem<T, U> {
    network: Rc<RefCell<Network<T>>>,
    invalid: HashSet<U>,
}

impl<T: Debug, U: Premise> TruthManagementSystem<T, U> {
    pub fn new(network: &Rc<RefCell<Network<T>>>) -> Self {
        Self {
            network: Rc::clone(network),
            invalid: HashSet::new()
        }
    }

    pub fn premise_is_valid(&self, premise: &U) -> bool {
        !self.invalid.contains(premise)
    }

    pub fn kick_out_premise(&mut self, premise: U) {
        self.invalid.remove(&premise);
    }

    pub fn bring_in_premise(&mut self, premise: U) {
        if self.invalid.contains(&premise) {
            self.network.borrow_mut().alert_all_propagators();
        }

        self.invalid.remove(&premise);
    }
}
