use crate::propagator_networks::cell;
use crate::propagator_networks::cell::{Cell, Merge};
use crate::propagator_networks::propagator;
use crate::propagator_networks::propagator::{Propagator};

use core::fmt::Debug;
//use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Network<A> {
    cells: Vec<Cell<A>>,
    propagators: Vec<Propagator<A>>,

    edges: HashMap<propagator::ID, Vec<cell::ID>>,

    alerted: Vec<propagator::ID>
}

impl<A> Network<A>
    where A: Debug + Merge + Clone + Copy + PartialEq
{
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            propagators: Vec::new(),

            edges: HashMap::new(),

            alerted: Vec::new()
        }
    }
    pub fn new_cell(&mut self) -> cell::ID {
        self.cells.push(Cell::new());
        self.cells.len() - 1
    }

    pub fn new_propagator(&mut self, propagator: Propagator<A>, cell_ids: &[cell::ID]) -> propagator::ID {
        self.propagators.push(propagator);

        let id = self.propagators.len() - 1;

        self.alerted.push(id);
        self.edges.insert(id, cell_ids.into());

        id
    }

    pub fn read_cell(&self, id: cell::ID) -> Option<A> {
        self.cells[id].to_option()
    }

    pub fn write_cell(&mut self, id: cell::ID, value: A) {
        let cell = &self.cells[id];
        let merged_cell = cell.merge(&Cell::wrap(value));

        merged_cell.assert_ok();

        self.cells[id] = merged_cell;
    }

    pub fn run(&mut self) {
        while self.alerted.len() > 0 {
            let mut writes : Vec<(cell::ID, A)>= Vec::new();

            for &prop_id in self.alerted.iter() {
                let propagator = &self.propagators[prop_id];
                let cell_ids = self.edges.get(&prop_id).unwrap();
                let input_cells : Vec<&Cell<A>> = cell_ids
                    .iter()
                    .take(cell_ids.len() - 1)
                    .map(|&cell_id| { &self.cells[cell_id] })
                    .collect();

                let &output_id = cell_ids.last().unwrap();
                let is_ready = input_cells.iter().all(|cell| !cell.is_empty());

                if is_ready {
                    let values : Vec<A> = input_cells
                        .iter()
                        .map(|&cell| cell.unwrap())
                        .collect();

                    let value = propagator.run(&values);

                    writes.push((output_id, value));
                }
            }

            self.alerted.clear();

            for (output_id, output) in writes.iter() {
                self.write_cell(*output_id, output.clone());
            }
        }
    }
}
