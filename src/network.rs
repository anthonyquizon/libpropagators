use crate::cell::{Merge, Cell};
use crate::propagator;
use crate::propagator::{Propagator, Procedure};
use crate::util::{CellID, PropagatorID};

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;

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

    pub fn run(&mut self) {
        while self.alerted.len() > 0 {
            let mut writes : Vec<(CellID, T)>= Vec::new();

            for &propagator_id in self.alerted.iter() {
                let propagator = &self.propagators[propagator_id];

                propagator.invoke(|&cell_id| {
                    self.cells[cell_id].read()
                });
                
            }
        }
    }
}


//impl<T> Network<T>
    //where A: Debug + Merge + Clone + PartialEq
//{
    //pub fn new() -> Self {
        //Self {
            //cells: Vec::new(),
            //propagators: Vec::new(),

            //alerted: HashSet::new()
        //}
    //}

    //pub fn label_cell(&mut self, id: cell::ID, label: &str) {
        //self.cells[id].set_label(label);
    //}

    //pub fn label_propagator(&mut self, id: cell::ID, label: &str) {
        //self.propagators[id].set_label(label)
    //}

    //pub fn make_cell(&mut self) -> cell::ID {
        //self.cells.push(Cell::new());
        //self.cells.len() - 1
    //}

    //pub fn alert_all_propagators(&mut self) {
        //for id in 0..self.propagators.len() - 1 {
            //self.alerted.insert(id);
        //}
    //}

    //pub fn make_propagator(&mut self, propagator: Propagator<T>, cell_ids: &[cell::ID]) -> propagator::ID {
        //self.propagators.push(propagator);

        //let id = self.propagators.len() - 1;

        //self.alerted.insert(id);
        //self.propagator_neighbours.insert(id, cell_ids.into());

        //for &cell_id in cell_ids {
            //match self.cell_neighbours.get_mut(&cell_id) {
                //Some(neighbours) => neighbours.push(id),
                //None => {
                    //let mut neighbours = Vec::new();

                    //neighbours.push(id);

                    //self.cell_neighbours.insert(cell_id, neighbours);
                //}
            //}
        //}

        //id
    //}

    //pub fn read_cell(&self, id: cell::ID) -> Option<T> {
        //self.cells[id].to_option()
    //}

    //pub fn write_cell(&mut self, id: cell::ID, value: T) {
        //let cell = &mut self.cells[id];

        //match cell.merge(&value) {
            //Event::Unchanged => {}
            //Event::Contradiction => { panic!("Contradiction!"); }
            //Event::Changed => {
                //match self.cell_neighbours.get(&id) {
                    //Some(neighbours) => {
                        //for &prop_id in neighbours.iter() {
                            //self.alerted.insert(prop_id);
                        //}
                    //}
                    //None => {}
                //};
            //}
        //}
    //}

    //pub fn num_cells(&self) -> usize {
        //self.cells.len()
    //}

    //pub fn abort_process(&mut self) {
        //self.alerted.clear();
    //}

    //pub fn run(&mut self) {
        //while self.alerted.len() > 0 {
            //let mut writes : Vec<(cell::ID, A)>= Vec::new();

            //for &prop_id in self.alerted.iter() {
                //let propagator = &self.propagators[prop_id];
                //let cell_ids = self.propagator_neighbours.get(&prop_id).unwrap();
                //let input_cells : Vec<&Cell<T>> = cell_ids
                    //.iter()
                    //.take(cell_ids.len() - 1)
                    //.map(|&cell_id| { &self.cells[cell_id] })
                    //.collect();

                //let &output_id = cell_ids.last().unwrap();
                //let is_ready = input_cells.iter().all(|&cell| !cell.is_empty());

                //if is_ready {
                    //let values : Vec<T> = input_cells
                        //.iter()
                        //.map(|&cell| cell.unwrap())
                        //.collect();

                    //let value = propagator.run(&values);

                    //writes.push((output_id, value));
                //}
            //}

            //self.alerted.clear();

            //for (output_id, output) in writes.iter() {
                //self.write_cell(*output_id, output.clone());
            //}
        //}
    //}
//}
