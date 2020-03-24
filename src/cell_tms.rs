use std::rc::{Rc};
use std::hash::{Hash, Hasher};
use crate::cell::{ Merge };
use crate::cell_supported::{ Supported };
use crate::tms::{ TruthManagementSystem, Premise };

use std::collections::HashSet;


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
    pub fn new(
        tms: &Rc<TruthManagementSystem<TruthManagementStore<A>>>, 
        value: A,
        premises: &[Premise]
    ) -> Self {
        let mut supports = HashSet::new();
        supports.insert(Supported::new(value, premises));

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
            instance.get_premises().iter().all(|premise| {
                self.system.premise_is_valid(premise.clone())
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

        // FIXME
        (*answer.get_value()).clone()
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
