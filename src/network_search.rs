use crate::propagator::{Propagator};
use crate::cell;
use crate::cell::{Merge};
use crate::network::{Network};
use std::fmt::Debug;
use crate::tms::{ TruthManagementSystem, Premise };


impl<A> Network<A> where A: Merge + Debug +  Clone + PartialEq
{
    pub fn propagator_add(&mut self, tms: Rc<TruthManagementSystem<TruthManagementStore<A>>>, a: cell::ID) {
        let prop = Propagator::Binary(|a, b| a + b);

        self.make_propagator(prop, &[a, b, c]);
    }
}


