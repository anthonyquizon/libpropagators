
use propagator::{Propagator};
use domain::{Domain};
use variable::{Variable};


pub struct Scheduler {
    variables: Vec<Variable>,
    propagators: Vec<Propagator>,
    //agenda: Agenda
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
            propagators: Vec::new(),
            //agenda
        }
    }

    pub fn propagate() {
        //for each in agenda
    }
}


