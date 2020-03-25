use crate::propagator::{Propagator};
use crate::cell;
use crate::cell::{Merge};
use crate::network::{Network};
use std::ops::{ Add, Sub, Mul, Div };


impl<A> Network<A> where A: Merge + Clone + PartialEq + Add<Output=A>
{
    pub fn propagator_add(&mut self, a: cell::ID, b: cell::ID, c: cell::ID) {
        let prop = Propagator::Binary(|a, b| a + b);

        self.make_propagator(prop, &[a, b, c]);
    }
}

impl<A> Network<A> where A: Merge + Clone + PartialEq + Sub<Output=A> {
    pub fn propagator_subtract(&mut self, a: cell::ID, b: cell::ID, c: cell::ID) {
        let prop = Propagator::Binary(|a, b| a - b);

        self.make_propagator(prop, &[a, b, c]);
    }
}

impl<A> Network<A> where A: Merge + Clone + PartialEq + Mul<Output=A> {
    pub fn propagator_multiply(&mut self, a: cell::ID, b: cell::ID, c: cell::ID) {
        let prop = Propagator::Binary(|a, b| a * b);

        self.make_propagator(prop, &[a, b, c]);
    }
}

impl<A> Network<A> where A: Merge + Clone + PartialEq + Div<Output=A> {
    pub fn propagator_divide(&mut self, a: cell::ID, b: cell::ID, c: cell::ID) {
        let prop = Propagator::Binary(|a, b| a / b);

        self.make_propagator(prop, &[a, b, c]);
    }
}

impl<A> Network<A> where A: Merge + Clone + PartialEq + Add<Output=A> + Sub<Output=A> {
    pub fn constraint_add(&mut self, a: cell::ID, b: cell::ID, c: cell::ID) {
        self.propagator_add(a, b, c);
        self.propagator_subtract(c, a, b);
        self.propagator_subtract(c, b, a);
    }
}

impl<A> Network<A> where A: Merge + Clone + PartialEq + Mul<Output=A> + Div<Output=A> {
    pub fn constraint_product(&mut self, a: cell::ID, b: cell::ID, c: cell::ID) {
        self.propagator_multiply(a, b, c);
        self.propagator_divide(c, a, b);
        self.propagator_divide(c, b, a);
    }

    pub fn constraint_similar_triangles(&mut self, a: cell::ID, b: cell::ID, c: cell::ID, d: cell::ID) {
        let ratio = self.make_cell();

        self.constraint_product(a, ratio, b);
        self.constraint_product(c, ratio, d);
    }
}
