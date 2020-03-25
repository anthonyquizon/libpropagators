use std::rc::{Rc};
use std::hash::{Hash, Hasher};
use crate::cell::{ Merge };
use crate::tms::{ TruthManagementSystem, Premise };
use std::ops::{ Add, Sub, Mul, Div };
use core::fmt::Debug;
use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Supported<A> {
    value: A,
    premises: HashSet<Premise>
}

impl<A: Hash> Hash for Supported<A> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        let mut vec_premises: Vec<&Premise> = self.premises.iter().collect();
        vec_premises.sort();

        self.value.hash(state);

        for dependency in vec_premises.iter() {
            dependency.hash(state);
        }
    }
}

impl<A: Add<Output = A>> Add for Supported<A> {
    type Output = Supported<A>;

    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value,
            premises: self.premises.union(&other.premises).cloned().collect()
        }
    }
}

impl<A: Sub<Output = A>> Sub for Supported<A> {
    type Output = Supported<A>;

    fn sub(self, other: Self) -> Self {
        Self {
            value: self.value - other.value,
            premises: self.premises.union(&other.premises).cloned().collect()
        }
    }
}

impl<A: Mul<Output = A>> Mul for Supported<A> {
    type Output = Supported<A>;

    fn mul(self, other: Self) -> Self {
        Self {
            value: self.value * other.value,
            premises: self.premises.union(&other.premises).cloned().collect()
        }
    }
}

impl<A: Div<Output = A>> Div for Supported<A> {
    type Output = Supported<A>;

    fn div(self, other: Self) -> Self {
        Self {
            value: self.value / other.value,
            premises: self.premises.union(&other.premises).cloned().collect()
        }
    }
}


impl<A: Merge + PartialEq> Supported<A> {
    pub fn new(value: A, premises_arr: &[Premise]) -> Self {
        let mut premises = HashSet::new();

        for premise in premises_arr {
            premises.insert(premise.clone());
        }

        Self {
            value,
            premises
        }
    }

    pub fn implies(&self, other: &Self) -> bool {
        let value = self.value.merge(&other.value);

        self.value == value
    }

    pub fn subsumes(&self, other: &Self) -> bool {
        self.implies(other) && self.premises.is_subset(&other.premises)
    }

    pub fn value(&self) -> &A {
        &self.value
    }

    pub fn premises(&self) -> &HashSet<Premise> {
        &self.premises
    }
}

impl<A: Merge + Clone + PartialEq> Merge for Supported<A> {
    fn is_valid(&self, other: &Self) -> bool {
        self.value.is_valid(&other.value)
    }

    fn merge(&self, other: &Self) -> Self {
        let merged_value = self.value.merge(&other.value);

        if merged_value == self.value {
            if other.value.merge(&merged_value) == other.value {
                if self.premises != other.premises && 
                    other.premises.is_subset(&self.premises) 
                {
                    (*other).clone()
                } else {
                    (*self).clone()
                }
            }
            else {
                (*self).clone()
            }
        }
        else if merged_value == other.value {
            (*other).clone()
        }
        else {
            Self {
                value: merged_value,
                premises: self.premises.union(&other.premises).cloned().collect()
            }
        }
    }
}


