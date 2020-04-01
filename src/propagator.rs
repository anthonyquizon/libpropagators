use crate::util::CellID;

use std::collections::HashSet;

pub enum Procedure<T> {
    Unary(fn(&T) -> T),
    Binary(fn(&T, &T) -> T),
    Ternary(fn(&T, &T, &T) -> T),
}

pub struct Propagator<T> {
    pub label: String,
    procedure: Procedure<T>,
    neighbours: HashSet<CellID>
}

impl<T> Propagator<T> {
    pub fn new(procedure: Procedure<T>) -> Self {
        Self {
            label: String::from(""),
            procedure,
            neighbours: HashSet::new()
        }
    }

    pub fn run(&self, contents: &[T]) -> T {
        match &self.procedure {
            Procedure::Unary(proc) => proc(&contents[0]),
            Procedure::Binary(proc) => proc(&contents[0], &contents[1]),
            Procedure::Ternary(proc) => proc(&contents[0], &contents[1], &contents[2]),
        }
    }

    pub fn add_neighbour(&mut self, id: CellID) {
        self.neighbours.insert(id);
    }
}
