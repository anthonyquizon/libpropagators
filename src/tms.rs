use crate::network::{ Network };
use std::hash::{Hash};
use crate::cell::{Merge};
use std::collections::HashSet;
use std::rc::{Rc};
use std::cell::RefCell;
use std::fmt::Debug;

pub type Premise = String;

#[derive(Clone)]
pub struct TruthManagementSystem<A> {
    network: Rc<RefCell<Network<A>>>,
    invalid: HashSet<Premise>,
}

impl<A: Merge + Debug + Clone + PartialEq> TruthManagementSystem<A> {
    pub fn new(network: &Rc<RefCell<Network<A>>>) -> Self {
        Self {
            network: Rc::clone(network),
            invalid: HashSet::new()
        }
    }

    pub fn premise_is_valid(&self, premise: Premise) -> bool {
        !self.invalid.contains(&premise)
    }

    pub fn kick_out_premise(&mut self, premise: Premise) {
        self.invalid.remove(&premise);
    }

    pub fn bring_in_premise(&mut self, premise: Premise) {
        if self.invalid.contains(&premise) {
            self.network.borrow_mut().alert_all_propagators();
        }

        self.invalid.remove(&premise);
    }
}
