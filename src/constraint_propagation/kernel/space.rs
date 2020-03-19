
use domain::{Domain};
use propagator::{Propagator};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use variable;


/// Definition 5.1
/// A space S = <d', Q> is admissible for a propagator problem <d, P> if and
/// only if d' ⊆ d, and all propagators that are not on the agenda are at a 
/// fixed point ∀p ∈ P \ Q: p(d') = d'


pub struct Space {
    variables: Vec<Variable>,
    propagators: Vec<Propagator>
    //variables: Vec<variable::ID>,
    dependencies: HashMap<variable::ID, propagator::ID>,
    agenda: BinaryHeap<propagator::ID>
}

impl Space {
    //pub fn admissible(&self) -> bool {
    //}
}


