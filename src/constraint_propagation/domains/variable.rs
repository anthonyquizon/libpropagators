use domain;
use state::{State};

pub type ID = usize;

pub struct Variable {
    id: usize,
    label: String,
    domainId: domain::ID
}

impl Variable {
    pub fn min(&self, domains: Vec<Domain>) -> usize {
        domains[self.domainId].min()
    }

    pub fn max(&self, domains: Vec<Domain>)  -> usize {
        domains[self.domainId].max()
    }

    //TODO return event
    pub fn set_min(&self, domains: Vec<Domain>, min: usize) -> &State {
        domains[self.domainId].set_max(min)
    }

    pub fn set_max(&self, domains: Vec<Domain>, max: usize) -> &State {
        domains[self.domainId].set_max(max)
    }
}
