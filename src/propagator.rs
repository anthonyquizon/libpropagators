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
    neighbours: Vec<CellID>
}

impl<T> Propagator<T> {
    pub fn new(procedure: Procedure<T>) -> Self {
        Self {
            label: String::from(""),
            procedure,
            neighbours: Vec::new()
        }
    }

    pub fn input_ids(&self) -> impl Iterator<Item = &CellID> {
        let n = self.neighbours.len();
        self.neighbours.iter().take(n - 1)
    }


    pub fn invoke(&self, inputs: &[&T]) -> T {
        let n = self.neighbours.len();

        match &self.procedure {
            Procedure::Unary(proc) => proc(&inputs[0]),
            Procedure::Binary(proc) => proc(&inputs[0], &inputs[1]),
            Procedure::Ternary(proc) => proc(&inputs[0], &inputs[1], &inputs[2]),
        }
    }

    pub fn add_neighbour(&mut self, id: CellID) {
        self.neighbours.push(id);
        self.neighbours.sort_unstable();
        self.neighbours.dedup();
    }
}
