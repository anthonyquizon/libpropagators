use crate::cell::{Merge};


pub type ID = usize;

#[derive(Clone)]
pub enum Propagator<A> {
    Unary(fn(&A) -> A),
    Binary(fn(&A, &A) -> A),
    Ternary(fn(&A, &A, &A) -> A),
}

impl<A: Merge + Clone> Propagator<A> {
    pub fn run(&self, values: &[A]) -> A {
        match &self {
            Self::Unary(proc) => proc(&values[0]),
            Self::Binary(proc) => proc(&values[0], &values[1]),
            Self::Ternary(proc) => proc(&values[0], &values[1], &values[2]),
        }
    }
}
