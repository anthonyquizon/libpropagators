use crate::util::CellID;
use crate::cell::Merge;

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

    pub fn add_neighbour(&mut self, id: CellID) {
        self.neighbours.push(id);
        self.neighbours.sort_unstable();
        self.neighbours.dedup();
    }
}

impl<T: Merge> Propagator<T> {
    pub fn invoke<'a, F: Fn(&CellID) -> &'a T>(&'a self, read: F) -> Option<(CellID, T)> {
        let n_inputs = self.neighbours.len() - 1;
        let &output_id = self.neighbours.last().unwrap();
        let inputs : Vec<&T> = self.neighbours
            .iter()
            .take(n_inputs)
            .map(read)
            .collect();

        if !inputs.iter().all(|&cell| !cell.is_empty()) {
            return None;
        }

        let output = match &self.procedure {
            Procedure::Unary(proc) => proc(&inputs[0]),
            Procedure::Binary(proc) => proc(&inputs[0], &inputs[1]),
            Procedure::Ternary(proc) => proc(&inputs[0], &inputs[1], &inputs[2]),
        };

        Some((output_id, output))
    }
}
