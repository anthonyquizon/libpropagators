use crate::cell::{ Merge };

use std::cmp;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Interval {
    min: i32,
    max: i32
}

impl Merge for Interval {
    fn is_valid(&self, _other: &Self) -> bool { true }

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

