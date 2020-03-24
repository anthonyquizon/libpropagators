use std::rc::{Rc};
use std::hash::{Hash, Hasher};
use crate::cell::{ Merge };
use crate::tms::{ TruthManagementSystem, Premise };
use core::fmt::Debug;

use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Supported<A> {
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

impl<A: Merge + PartialEq> Supported<A> {
    pub fn new(value: A) -> Self {
        Self {
            value,
            premises: HashSet::new()
        }
    }

    pub fn implies(&self, other: &Self) -> bool {
        let value = self.value.merge(&other.value);

        self.value == value
    }

    pub fn subsumes(&self, other: &Self) -> bool {
        self.implies(other) && self.premises.is_subset(&other.premises)
    }
}

impl<A: Merge + Clone + PartialEq> Merge for Supported<A> {
    fn is_valid(&self, _other: &Self) -> bool { true }

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

#[derive(Clone)]
pub struct TruthManagementStore<A> {
    system: Rc<TruthManagementSystem<TruthManagementStore<A>>>,
    supports: HashSet<Supported<A>>,
}

impl<A: PartialEq + Eq + Hash> PartialEq for TruthManagementStore<A> {
    fn eq(&self, other: &Self) -> bool {
        self.supports == other.supports
    }
}

impl<A: Clone + Hash + Merge + PartialEq + Eq> TruthManagementStore<A> {
    pub fn new(tms: &Rc<TruthManagementSystem<TruthManagementStore<A>>>, initial: A) -> Self {
        let mut supports = HashSet::new();
        supports.insert(Supported::new(initial));

        Self {
            system: Rc::clone(tms),
            supports
        }
    }

    fn from_supports(tms: &Rc<TruthManagementSystem<TruthManagementStore<A>>>, supports: HashSet<Supported<A>>) -> Self {
        Self {
            system: Rc::clone(tms),
            supports
        }
    }

    /// Pg 54
    fn assimilate_many(&self, other_supports: &HashSet<Supported<A>>) -> Self {
        let mut tms = self.clone();

        for other_instance in other_supports.iter() {
            tms = tms.assimilate(other_instance);
        }

        tms
    }

    fn assimilate(&self, other_instance: &Supported<A>) -> Self {
        let mut supports : HashSet<Supported<A>> = HashSet::new();

        let any_subsumes = self.supports.iter().any(|self_instance| {
            self_instance.subsumes(other_instance)
        });

        if !any_subsumes {
            let subsumed : HashSet<Supported<A>> = self.supports.iter()
                .cloned()
                .filter(|self_instance| other_instance.subsumes(&self_instance))
                .collect();

            supports = supports
                .difference(&subsumed)
                .cloned()
                .collect();

            supports.insert(other_instance.clone());
        }

        Self::from_supports(&self.system, supports)
    }

    fn strongest_consequence(&self) -> Supported<A> {
        let relevant_supports : Vec<&Supported<A>> = self.supports.iter().filter(|instance| {
            instance.premises.iter().all(|&premise| {
                self.system.premise_is_valid(premise)
            })
        }).collect();

        let head : Supported<A>= (*relevant_supports.first().unwrap()).clone();
        let tail : Vec<&Supported<A>> = relevant_supports
            .iter()
            .cloned()
            .skip(1)
            .collect();

        tail.iter().fold(head, |acc, &support| {
            acc.merge(support)
        })
    }

    pub fn query(&mut self) -> A {
        let answer = self.strongest_consequence();
        let better_tms = self.assimilate(&answer);

        if *self != better_tms { 
            self.supports = better_tms.supports;
        }

        answer.value
    }
}

impl<A: Clone + Hash + Merge + PartialEq + Eq> Merge for TruthManagementStore<A> {
    fn is_valid(&self, _other: &Self) -> bool { true }

    fn merge(&self, other: &Self) -> Self {
        let candidate = self.assimilate_many(&other.supports);
        let consequence = candidate.strongest_consequence();

        self.assimilate(&consequence)
    }
}
