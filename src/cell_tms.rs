use std::rc::{Rc};
use std::fmt::{Debug, Formatter, Result};
use crate::cell::{ Merge };
use crate::cell_supported::{ Supported };
use std::ops::{ Add, Sub, Mul, Div };
use crate::tms::{ TruthManagementSystem, Premise };


#[derive(Clone)]
pub struct TruthManagementStore<A> {
    system: Rc<TruthManagementSystem<TruthManagementStore<A>>>,
    supports: Vec<Supported<A>>,
}

impl<A: Debug> Debug for TruthManagementStore<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Truth Management Store")
         .field("supports", &self.supports)
         .finish()
    }
}

impl<A: PartialEq> PartialEq for TruthManagementStore<A> {
    fn eq(&self, other: &Self) -> bool {
        self.supports == other.supports
    }
}

impl<A: Clone + Add<Output = A>> Add for TruthManagementStore<A> {
    type Output = TruthManagementStore<A>;

    fn add(self, other: Self) -> Self {
        let mut supports = Vec::new();

        for self_support in self.supports.iter() {
            for other_support in other.supports.iter() {
                let support = (*self_support).clone() + (*other_support).clone();
                
                supports.push(support);
            }
        }

        Self {
            system: Rc::clone(&self.system),
            supports
        }
    }
}

impl<A: Clone + Sub<Output = A>> Sub for TruthManagementStore<A> {
    type Output = TruthManagementStore<A>;

    fn sub(self, other: Self) -> Self {
        let mut supports = Vec::new();

        //merge
        for self_support in self.supports.iter() {
            for other_support in other.supports.iter() {
                //FIXME remove clones
                let support = (*self_support).clone() - (*other_support).clone();
                supports.push(support);
            }
        }

        Self {
            system: Rc::clone(&self.system),
            supports
        }
    }
}

//impl<A: Mul<Output = A>> Mul for Supported<A> {
    //type Output = Supported<A>;

    //fn mul(self, other: Self) -> Self {
        //Self {
            //value: self.value * other.value,
            //premises: self.premises.union(&other.premises).cloned().collect()
        //}
    //}
//}

//impl<A: Div<Output = A>> Div for Supported<A> {
    //type Output = Supported<A>;

    //fn div(self, other: Self) -> Self {
        //Self {
            //value: self.value / other.value,
            //premises: self.premises.union(&other.premises).cloned().collect()
        //}
    //}
//}

impl<A: Debug + Clone + Merge + PartialEq> TruthManagementStore<A> {
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
        // If you can get the same value from any of the current supports
        // while only using a subset of the premises, ie. you require less 
        // information to get to the same answer, then return the current tms
        let any_subsumes = self.supports.iter().any(|supported| {
            supported.subsumes(&other_supported)
        });

            println!("===assimilate===");
            println!("any subsumes {:?}", any_subsumes);
            println!("===============");

        if any_subsumes {
            (*self).clone()
        }
        else {
            let mut supports : Vec<Supported<A>>= self.supports.iter()
                .cloned()
                .filter(|supported| !other_supported.subsumes(&supported))
                .collect();

            println!("===assimilate===");
            println!("other {:?}", other_supported);
            println!("supports {:?}", supports);
            println!("===============");
            // FIXME: remove cloned
            let exists = supports
                .iter()
                .cloned()
                .all(|supported| supported == *other_supported);

            if !exists {
                supports.push(other_supported.clone());
            }

            Self::from_supports(&self.system, supports)
        }
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

    pub fn supports(&self) -> &Vec<Supported<A>> {
        &self.supports
    }
}

impl<A: Debug + Clone + Merge + PartialEq> Merge for TruthManagementStore<A> {
    fn merge(&self, other: &Self) -> Self {
        let candidate = self.assimilate_many(&other.supports);
        println!("===merge===");
        println!("{:?}", self);
        println!("{:?}", other);
        println!("{:?}", candidate);
        println!("==========");

        if candidate.supports.len() > 2 {
            return self.clone();
        }
        let consequence = candidate.strongest_consequence();

        self.assimilate(&consequence)
    }
}
