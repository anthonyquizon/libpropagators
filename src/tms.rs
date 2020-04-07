use crate::network::{ Network };
use crate::premise::Premise;
use crate::context::Context;
use std::collections::HashSet;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;
use std::fmt::Debug;


#[derive(Clone)]
pub struct TMSContext<T, U> {
    system: Rc<RefCell<TruthManagementSystem<T, U>>>,
}

impl<T: Clone + Debug, U: Clone + Debug> Context for TMSContext<T, U> {}

impl<T, U> fmt::Debug for TMSContext<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TMS Context")
    }
}


#[derive(Clone)]
pub struct TruthManagementSystem<T, U> {
    network: Rc<RefCell<Network<TMSContext<T, U>, T>>>,
    premise_outness: HashSet<U>,
    premise_nogoods: HashMap<U, Vec<U>>,
}

impl<T: Debug, U: Premise> TruthManagementSystem<T, U> {
    pub fn new(network: &Rc<RefCell<Network<TMSContext<T, U>, T>>>) -> Self {
        Self {
            network: Rc::clone(network),
            premise_outness: HashSet::new(),
            premise_nogoods: HashMap::new()
        }
    }

    pub fn premise_in(&self, premise: &U) -> bool {
        !self.premise_outness.contains(premise)
    }

    pub fn reasons_against_premise(&self, premise: &U) -> Option<impl Iterator<Item=&U>> {
        self.premise_nogoods.get(premise).map(|vec| vec.iter())
    }

    pub fn kick_out_premise(&mut self, premise: U) {
        if self.premise_outness.contains(&premise) {
            self.premise_outness.remove(&premise);
            self.network.borrow_mut().alert_all_propagators();
        }
    }

    pub fn bring_in_premise(&mut self, premise: U) {
        if !self.premise_outness.contains(&premise) {
            self.premise_outness.insert(premise);
            self.network.borrow_mut().alert_all_propagators();
        }
    }
}
