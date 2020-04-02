use crate::propagator::{Procedure};
use crate::util::{ CellID, PropagatorID };
use crate::network::Network;
use std::ops::{ Add, Sub, Mul, Div };
use std::fmt::Debug;


impl<T: Add<Output=T>> Network<T> {
    pub fn propagator_add(&mut self, a: CellID, b: CellID, c: CellID) {
        let proc = Procedure::Binary(|a: T, b: T| a + b);

        let id = self.make_propagator(proc, &[a, b], c);

        self.label_propagator(id, "add");
    }
}

impl<T: Sub<Output=T>> Network<T>  {
    pub fn propagator_subtract(&mut self, a: CellID, b: CellID, c: CellID) {
        let prop = Procedure::Binary(|a, b| a - b);

        let id = self.make_propagator(prop, &[a, b], c);

        self.label_propagator(id, "sub");
    }
}

impl<T: Mul<Output=T>> Network<T> {
    pub fn propagator_multiply(&mut self, a: CellID, b: CellID, c: CellID) {
        let prop = Procedure::Binary(|a, b| a * b);

        let id = self.make_propagator(prop, &[a, b], c);
        
        self.label_propagator(id, "mul");
    }
}

impl<T: Div<Output=T>> Network<T> {
    pub fn propagator_divide(&mut self, a: CellID, b: CellID, c: CellID) {
        let prop = Procedure::Binary(|a, b| a / b);

        let id = self.make_propagator(prop, &[a, b], c);

        self.label_propagator(id, "div");
    }
}

impl<T: Add<Output=T> + Sub<Output=T>> Network<T> {
    pub fn constraint_add(&mut self, a: CellID, b: CellID, c: CellID) {
        self.propagator_add(a, b, c);
        self.propagator_subtract(c, a, b);
        self.propagator_subtract(c, b, a);
    }
}

impl<T: Mul<Output=T> + Div<Output=T>> Network<T> {
    pub fn constraint_product(&mut self, a: CellID, b: CellID, c: CellID) {
        self.propagator_multiply(a, b, c);
        self.propagator_divide(c, a, b);
        self.propagator_divide(c, b, a);
    }
}

impl<T: Default + Mul<Output=T> + Div<Output=T>> Network<T> {
    pub fn constraint_similar_triangles(&mut self, a: CellID, b: CellID, c: CellID, d: CellID) {
        let ratio = self.make_cell();

        self.label_cell(ratio, "ratio");

        self.constraint_product(a, ratio, b);
        self.constraint_product(c, ratio, d);
    }
}
