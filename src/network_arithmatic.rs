use crate::propagator::{Propagator};
use crate::cell;
use crate::cell::{Merge};
use crate::network::{Network};
use std::ops::{ Add, Sub };

impl<A> Network<A> 
where A: Copy + Merge + Clone + Add<Output=A> + Sub<Output=A>
{
    pub fn propagator_add(&mut self, a: cell::ID, b: cell::ID, c: cell::ID) {
        let prop = Propagator::Binary(|&a, &b| a + b);

        self.new_propagator(prop, &[a, b, c]);
    }

    pub fn propagator_sub(&mut self, a: cell::ID, b: cell::ID, c: cell::ID) {
        let prop = Propagator::Binary(|&a, &b| a - b);

        self.new_propagator(prop, &[a, b, c]);
    }

    pub fn constraint_add(&mut self, a: cell::ID, b: cell::ID, c: cell::ID) {
        self.propagator_add(a, b, c);
        self.propagator_sub(c, b, a);
        self.propagator_sub(c, a, b);
    }
}

