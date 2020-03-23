use crate::network::{ Network };
use std::collections::HashSet;
use std::rc::{Rc};

pub type Premise = usize;

#[derive(Clone)]
pub struct TruthManagementSystem<A> {
    network: Rc<Network<A>>,
    invalid: HashSet<Premise>,
}

impl<A> TruthManagementSystem<A> {
    pub fn new(network: &Rc<Network<A>>) -> Self {
        Self {
            network: Rc::clone(network),
            invalid: HashSet::new()
        }
    }
}
