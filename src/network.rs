use crate::cell::{Merge, Cell};
use crate::propagator::{Propagator, Procedure};
use crate::util::{CellID, PropagatorID};

use std::collections::HashSet;

#[derive(Default)]
pub struct Network<T> {
    cells: Vec<Cell<T>>,
    propagators: Vec<Propagator<T>>,

    alerted: HashSet<PropagatorID>
}

impl<T: Default> Network<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn make_cell(&mut self) -> CellID {
        let cell = Cell::new();

        self.cells.push(cell);

        let id = self.cells.len() - 1;

        id
    }
}

impl<T> Network<T> {
    pub fn make_propagator(&mut self, proc: Procedure<T>, cell_ids: &[CellID]) -> PropagatorID {
        let mut propagator = Propagator::new(proc);

        for &cell_id in cell_ids {
            propagator.add_neighbour(cell_id);
        }

        self.propagators.push(propagator);

        let id = self.propagators.len() - 1;

        for &cell_id in cell_ids {
            self.cells[cell_id].add_neighbour(id);
        }

        id
    }

    pub fn read_cell(&self, id: CellID) -> &T {
        self.cells[id].read()
    }
}

impl<T: Merge + PartialEq> Network<T> {
    pub fn write_cell(&mut self, id: CellID, value: T) {
        let cell = &mut self.cells[id];
        let alerted = cell.write(value);

        if let Some(alerted) = alerted {
            for &propagator_id in alerted.iter() {
                self.alerted.insert(propagator_id);
            }
        }

    }
}

impl<T: Merge + PartialEq> Network<T> {
    pub fn run(&mut self) {
        while self.alerted.len() > 0 {
            let mut writes : Vec<(CellID, T)>= Vec::new();

            for &propagator_id in self.alerted.iter() {
                let propagator = &self.propagators[propagator_id];

                let write_val = propagator.invoke(|&cell_id| {
                    self.cells[cell_id].read()
                });

                if let Some(write_val) = write_val {
                    writes.push(write_val);
                }
            }

            self.alerted.clear();

            writes.drain(0..).for_each(|(output_id, output)| {
                self.write_cell(output_id, output);
            });
        }
    }
}

