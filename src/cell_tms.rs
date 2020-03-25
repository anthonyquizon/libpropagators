use std::rc::{Rc};
use std::hash::{Hash, Hasher};
use crate::cell::{ Merge };
use crate::cell_supported::{ Supported };
use crate::tms::{ TruthManagementSystem, Premise };


#[derive(Clone)]
pub struct TruthManagementStore<A> {
    system: Rc<TruthManagementSystem<TruthManagementStore<A>>>,
    supports: Vec<Supported<A>>,
}

impl<A: PartialEq + Eq> PartialEq for TruthManagementStore<A> {
    fn eq(&self, other: &Self) -> bool {
        self.supports == other.supports
    }
}

impl<A: Clone + Merge + PartialEq + Eq> TruthManagementStore<A> {
    pub fn new(
        tms: &Rc<TruthManagementSystem<TruthManagementStore<A>>>, 
        value: A,
        premises: &[Premise]
    ) -> Self {
        Self {
            system: Rc::clone(tms),
            supports: vec![Supported::new(value, premises)]
        }
    }

    fn from_supports(tms: &Rc<TruthManagementSystem<TruthManagementStore<A>>>, supports: Vec<Supported<A>>) -> Self {
        Self {
            system: Rc::clone(tms),
            supports
        }
    }

    fn assimilate_many(&self, other_supports: &Vec<Supported<A>>) -> Self {
        let mut tms = self.clone();

        for other_supported in other_supports.iter() {
            tms = tms.assimilate(other_supported);
        }

        tms
    }

    fn assimilate(&self, other_supported: &Supported<A>) -> Self {
        let mut supports : Vec<Supported<A>> = Vec::new();

        let any_subsumes = self.supports.iter().any(|supported| {
            supported.subsumes(&other_supported)
        });

        if !any_subsumes {
            supports = self.supports.iter()
                .cloned()
                .filter(|supported| !other_supported.subsumes(&supported))
                .collect();

            // FIXME: remove cloned
            let exists = supports
                .iter()
                .cloned()
                .all(|supported| supported == *other_supported);

            if !exists {
                supports.push(other_supported.clone());
            }
        }

        Self::from_supports(&self.system, supports)
    }

    fn strongest_consequence(&self) -> Supported<A> {
        let relevant_supports : Vec<&Supported<A>> = self.supports.iter().filter(|instance| {
            instance.premises().iter().all(|premise| {
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
        (*answer.value()).clone()
    }
}

impl<A: Clone + Merge + PartialEq + Eq> Merge for TruthManagementStore<A> {
    fn is_valid(&self, _other: &Self) -> bool { true }

    fn merge(&self, other: &Self) -> Self {
        let candidate = self.assimilate_many(&other.supports);
        let consequence = candidate.strongest_consequence();

        self.assimilate(&consequence)
    }
}
