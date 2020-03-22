use crate::cell::{ Merge };


pub type Premise = usize;
use std::collections::HashSet;
use std::cmp;

// TODO Move to merge instances
//pub struct Instance {
//}

// TODO make generic?
pub struct Interval {
    min: i32,
    max: i32
}

pub struct IntervalDeps {
    intervals: Interval,
    dependencies: HashSet<Premise>
}

impl Merge for Interval {
    fn is_valid(&self, other: &Self) -> bool { true }

    fn merge(&self, other: &Self) -> Self {
        let min = cmp::max(self.min, other.min);
        let max = cmp::min(self.max, other.max);

        Self {
            min,
            max
        }
    }
}

impl Interval {
    pub fn new(min: i32, max: i32) -> Self {
        Self {
            min,
            max
        }
    }
}

//impl Merge for Interval {
    //fn is_valid(&self, value: &Self) -> bool {
        //self == value
    //}

    //fn merge(&self, other: &Self) -> Self {
        //value.clone()
    //}
//}
