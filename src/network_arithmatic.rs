use crate::propagator_networks::propagator::{Propagator};
use crate::propagator_networks::cell;
use crate::propagator_networks::cell::{Merge};
use crate::propagator_networks::network::{Network};
use std::ops::{ Add, Sub };
use core::fmt::Debug;

impl<A> Network<A> 
where A: Copy + Merge + Clone + Debug + PartialEq + Add<Output=A> + Sub<Output=A>
{
    pub fn propagator_add(&mut self, a: cell::ID, b: cell::ID, c: cell::ID) {
        let prop = Propagator::Binary(|a, b| a + b);

        self.new_propagator(prop, &[a, b, c]);
    }

    pub fn propagator_sub(&mut self, a: cell::ID, b: cell::ID, c: cell::ID) {
        let prop = Propagator::Binary(|a, b| a - b);

        self.new_propagator(prop, &[a, b, c]);
    }

    pub fn constraint_add(&mut self, a: cell::ID, b: cell::ID, c: cell::ID) {
        self.propagator_add(a, b, c);
        self.propagator_sub(c, b, a);
        self.propagator_sub(c, a, b);
    }
}

