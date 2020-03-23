use std::rc::{Rc};
use std::hash::{Hash, Hasher};
use crate::cell::{ Merge };
use crate::tms::{ TruthManagementSystem, Premise };
use core::fmt::Debug;

use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Instance<A> {
    value: A,
    dependencies: HashSet<Premise>
}

#[derive(Clone)]
pub struct TruthManagementStore<A> {
    system: Rc<TruthManagementSystem<TruthManagementStore<A>>>,
    instances: HashSet<Instance<A>>,
}

impl<A: Hash> Hash for Instance<A> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        let mut vec_dependencies: Vec<&Premise> = self.dependencies.iter().collect();
        vec_dependencies.sort();

        self.value.hash(state);

        for dependency in vec_dependencies.iter() {
            dependency.hash(state);
        }
    }
}

impl<A: Merge + PartialEq> Instance<A> {
    pub fn new(value: A) -> Self {
        Self {
            value,
            dependencies: HashSet::new()
        }
    }

    pub fn implies(&self, other: &Self) -> bool {
        let value = self.value.merge(&other.value);

        self.value == value
    }

    pub fn subsumes(&self, other: &Self) -> bool {
        self.implies(other) && self.dependencies.is_subset(&other.dependencies)
    }
}

impl<A: Clone + Hash + Merge + PartialEq + Eq> TruthManagementStore<A> {
    pub fn new(tms: &Rc<TruthManagementSystem<TruthManagementStore<A>>>, initial: A) -> Self {
        let mut instances = HashSet::new();
        instances.insert(Instance::new(initial));

        Self {
            system: Rc::clone(tms),
            instances
        }
    }

    /// Pg 54
    fn assimilate(&mut self, other: &Self) {
        let mut instances : HashSet<Instance<A>> = HashSet::new();

        for other_instance in other.instances.iter() {
            let any_subsumes = self.instances.iter().any(|self_instance| {
                self_instance.subsumes(&other_instance)
            });

            if !any_subsumes {
                let subsumed : HashSet<Instance<A>> = self.instances.iter()
                    .cloned()
                    .filter(|self_instance| other_instance.subsumes(&self_instance))
                    .collect();

                instances = instances
                    .difference(&subsumed)
                    .cloned()
                    .collect();

                instances.insert(other_instance.clone());
            }
        }

        self.instances = instances;
    }

    fn strongest_consequence(&self) {
        //let all_premises_in = 
    }
}

impl<A: Merge + PartialEq + Clone> Merge for TruthManagementStore<A> {
    fn is_valid(&self, _other: &Self) -> bool { true }

    fn merge(&self, other: &Self) -> Self {
        Self {
            system: Rc::clone(&self.system),
            instances: self.instances.clone()
        }
    }
}
