//use crate::content::{Content, Merge};


pub type ID = usize;

#[derive(Debug, Clone)]
pub enum Propagator<A> {
    Unary(fn(A) -> A),
    Binary(fn(A, A) -> A),
    Ternary(fn(A, A, A) -> A),
}
