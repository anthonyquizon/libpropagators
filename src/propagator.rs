use crate::node::Node;
use crate::util::{CellID};

pub enum Procedure<T> {
    Unary(fn(&T) -> T),
    Binary(fn(&T, &T) -> T),
    Ternary(fn(&T, &T, &T) -> T),
}

pub type Propagator<T> = Node<Procedure<T>, CellID>;

impl<T> Propagator<T> {
    pub fn run(&self, contents: &[T]) -> T {
        self.map(|proc| {
            match &proc {
                Procedure::Unary(proc) => proc(&contents[0]),
                Procedure::Binary(proc) => proc(&contents[0], &contents[1]),
                Procedure::Ternary(proc) => proc(&contents[0], &contents[1], &contents[2]),
            }
        })
    }
}
