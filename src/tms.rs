use crate::network::{ Network };
use crate::cell::{Merge};
use std::collections::HashSet;
use std::rc::{Rc};
use std::cell::RefCell;

pub type Premise = String;

#[derive(Clone)]
pub struct TruthManagementSystem<A> {
    network: Rc<RefCell<Network<A>>>,
    invalid: HashSet<Premise>,
}

impl<A: Merge + Clone + PartialEq> TruthManagementSystem<A> {
    pub fn new(network: &Rc<RefCell<Network<A>>>) -> Self {
        Self {
            network: Rc::clone(network),
            invalid: HashSet::new()
        }
    }

    pub fn premise_is_valid(&self, premise: Premise) -> bool {
        !self.invalid.contains(&premise)
    }

    pub fn invalidate_premise(&mut self, premise: Premise) {
        self.invalid.remove(&premise);
    }

    pub fn validate_premise(&mut self, premise: Premise) {
        if self.invalid.contains(&premise) {
            self.network.borrow_mut().alert_all_propagators();
        }

        self.invalid.remove(&premise);
    }
}
