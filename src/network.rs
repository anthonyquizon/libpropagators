use crate::cell;
use crate::cell::{Cell, Event, Merge};
use crate::propagator;
use crate::propagator::{Propagator};

//use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Network<A> {
    cells: Vec<Cell<A>>,
    propagators: Vec<Propagator<A>>,

    propagator_neighbours: HashMap<propagator::ID, Vec<cell::ID>>,
    cell_neighbours: HashMap<cell::ID, Vec<propagator::ID>>,

    alerted: HashSet<propagator::ID>
}

impl<A> Network<A>
    where A: Merge + Clone + PartialEq
{
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            propagators: Vec::new(),

            propagator_neighbours: HashMap::new(),
            cell_neighbours: HashMap::new(),

            alerted: HashSet::new()
        }
    }

    pub fn make_cell(&mut self) -> cell::ID {
        self.cells.push(Cell::new());
        self.cells.len() - 1
    }

    pub fn alert_all_propagators(&mut self) {
        for id in 0..self.propagators.len() - 1 {
            self.alerted.insert(id);
        }
    }

    pub fn make_propagator(&mut self, propagator: Propagator<A>, cell_ids: &[cell::ID]) -> propagator::ID {
        self.propagators.push(propagator);

        let id = self.propagators.len() - 1;

        self.alerted.insert(id);
        self.propagator_neighbours.insert(id, cell_ids.into());

        for &cell_id in cell_ids {
            match self.cell_neighbours.get_mut(&cell_id) {
                Some(neighbours) => neighbours.push(id),
                None => {
                    let mut neighbours = Vec::new();

                    neighbours.push(id);

                    self.cell_neighbours.insert(cell_id, neighbours);
                }
            }
        }

        id
    }

    pub fn read_cell(&self, id: cell::ID) -> Option<A> {
        self.cells[id].to_option()
    }

    pub fn write_cell(&mut self, id: cell::ID, value: A) {
        let cell = &self.cells[id];

        match cell.merge(&Cell::wrap(value)) {
            Event::Changed(merged_cell) => {
                match self.cell_neighbours.get(&id) {
                    Some(neighbours) => {
                        for &prop_id in neighbours.iter() {
                            self.alerted.insert(prop_id);
                        }
                    }
                    None => {}
                };

                self.cells[id] = merged_cell;
            }
            Event::Unchanged => {}
            Event::Contradiction => {
                panic!("Contradiction!");
            }
        }

    }

    pub fn num_cells(&self) -> usize {
        self.cells.len()
    }

    pub fn run(&mut self) {
        while self.alerted.len() > 0 {
            let mut writes : Vec<(cell::ID, A)>= Vec::new();

            for &prop_id in self.alerted.iter() {
                let propagator = &self.propagators[prop_id];
                let cell_ids = self.propagator_neighbours.get(&prop_id).unwrap();
                let input_cells : Vec<&Cell<A>> = cell_ids
                    .iter()
                    .take(cell_ids.len() - 1)
                    .map(|&cell_id| { &self.cells[cell_id] })
                    .collect();

                let &output_id = cell_ids.last().unwrap();
                let is_ready = input_cells.iter().all(|&cell| !cell.is_empty());

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
