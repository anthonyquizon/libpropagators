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

    prop_to_cells: HashMap<propagator::ID, Vec<cell::ID>>,
    cell_to_props: HashMap<cell::ID, Vec<propagator::ID>>,

    alerted: Vec<propagator::ID>
}

impl<A> Network<A>
    where A: Debug + Merge + Clone
{
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            propagators: Vec::new(),

            prop_to_cells: HashMap::new(),
            cell_to_props: HashMap::new(),

            alerted: Vec::new()
        }
    }
    pub fn add_cell(&mut self, cell: Cell<A>) -> cell::ID {
        self.cells.push(cell);
        self.cells.len() - 1
    }

    pub fn add_propagator(&mut self, propagator: Propagator<A>) -> propagator::ID {
        self.propagators.push(propagator);

        let id = self.propagators.len() - 1;

        self.alerted.push(id);

        id
    }

    pub fn read_cell(&self, id: cell::ID) -> &Cell<A> {
        &self.cells[id]
    }

    pub fn write_cell(&mut self, id: cell::ID, a: Cell<A>) {
        let cell = &self.cells[id];
        self.cells[id] = cell.merge(&a);
    }

    pub fn connect(&mut self, prop_id: propagator::ID, cell_id: cell::ID) {
        self.alerted.push(prop_id);

        match self.prop_to_cells.get_mut(&prop_id) {
            Some(cells) => {
                cells.push(cell_id);
            },
            None => {
                self.prop_to_cells.insert(prop_id, vec![cell_id]);
            }
        }

        match self.cell_to_props.get_mut(&cell_id) {
            Some(props) => {
                props.push(prop_id);
            },
            None => {
                self.cell_to_props.insert(cell_id, vec![prop_id]);
            }
        }

    }
/*
    pub fn run(&mut self) {
        while self.alerted.len() > 0 {
            let mut writes : Vec<(cell::ID, A)>= Vec::new();
            let alerted : Vec<propagator::Id> = Vec::new();

            for &prop_id in self.alerted.iter() {
                let propagator = self.propagators[prop_id];
                let cells = self.prop_to_cells.get(prop_id);
            }
        }
    }
    */
}
