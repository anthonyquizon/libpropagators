use crate::cell::Cell;
use crate::content::{ State, Merge };
use crate::propagator::{Propagator, Procedure, Return};
use crate::util::{CellID, PropagatorID};
use crate::context::Context;
use std::fmt::Debug;

use std::collections::HashSet;

#[derive(Default)]
pub struct Network<C: Context, T> {
    cells: Vec<Cell<T>>,
    propagators: Vec<Propagator<C, T>>,

    context: C,

    alerted: HashSet<PropagatorID>
}

impl<C: Context, T> Network<C, T> {
    pub fn new(context: C) -> Self {
        Self {
            cells: Vec::new(),
            propagators: Vec::new(),
            
            context,

            alerted: HashSet::new()
        }
    }
}

impl<C: Context, T: Default> Network<C, T> {
    pub fn make_cell(&mut self) -> CellID {
        let cell = Cell::new();

        self.cells.push(cell);

        let id = self.cells.len() - 1;

        id
    }
}

impl<C: Context, T> Network<C, T> {
    pub fn label_cell(&mut self, id: CellID, label: &str) {
        self.cells[id].label = String::from(label);
    }

    pub fn label_propagator(&mut self, id: PropagatorID, label: &str) {
        self.propagators[id].label = String::from(label);
    }

    pub fn make_propagator(&mut self, proc: Procedure<C, T>, inputs: &[CellID], output: CellID) -> PropagatorID {
        let propagator = Propagator::new(proc, inputs, output);

        self.propagators.push(propagator);

        let id = self.propagators.len() - 1;

        for &cell_id in inputs {
            self.cells[cell_id].add_neighbour(id);
        }

        self.alerted.insert(id);

        id
    }

    pub fn alert_all_propagators(&mut self) {
        for id in 0..self.propagators.len() - 1 {
            self.alerted.insert(id);
        }
    }

    pub fn read_cell(&self, id: CellID) -> &T {
        self.cells[id].read()
    }
}

impl<C: Context, T: Debug + Merge + PartialEq> Network<C, T> {
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

impl<C: Context, T> Network<C, T> 
where T: Debug + State + Clone + Merge + PartialEq {
    pub fn run(&mut self) {
        while self.alerted.len() > 0 {
            let mut writes : Vec<(CellID, Return<C, T>)> = Vec::new();

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
                match output {
                    Return::Pure(output) => self.write_cell(output_id, output),
                    Return::Action(action) => {
                        self.context.run_action(action);

                        self.alert_all_propagators();
                    },
                }
            });
        }
    }
}

