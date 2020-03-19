use crate::propagator::{Propagator};
use crate::content::{Merge};
use crate::network::{Network};
use std::ops::{ Add, Sub };
use core::fmt::Debug;

impl<A> Network<A> 
where A: Merge + Clone + Debug + PartialEq + Add<Output=A> + Sub<Output=A>
{
    fn prop_add(&mut self) {
        let prop = Propagator::Binary(|a, b| a + b);

        self.add_propagator(prop);
    }

    fn prop_sub(&mut self) {
        let prop = Propagator::Binary(|a, b| a - b);

        self.add_propagator(prop);
    }
}

