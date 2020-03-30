use crate::cell::{Merge};


pub type ID = usize;



#[derive(Clone)]
pub enum Propagator<A> {
    Unary(fn(A) -> A),
    Binary(fn(A, A) -> A),
    Ternary(fn(A, A, A) -> A),
}

impl<A: Merge + Clone> Propagator<A> {
    // FIXME: remove clones
    pub fn run(&self, values: &[A]) -> A {
        match &self {
            Propagator::Unary(proc) => proc(values[0].clone()),
            Propagator::Binary(proc) => proc(values[0].clone(), values[1].clone()),
            Propagator::Ternary(proc) => proc(values[0].clone(), values[1].clone(), values[2].clone()),
        }
    }
    
}
