use std::rc::{Rc};
use std::fmt::{Debug, Formatter, Result};
use crate::cell::{ Merge };
use crate::cell_supported::{ Supported };
use std::hash::{Hash};
use std::ops::{ Add, Sub, Mul, Div };
use crate::tms::{ TruthManagementSystem, Premise };
use std::collections::HashSet;


#[derive(Clone)]
pub struct TruthManagementStore<A> {
    system: Rc<TruthManagementSystem<TruthManagementStore<A>>>,
    supports: HashSet<Supported<A>>,
}

impl<A: Hash + PartialEq + Eq + Debug> Debug for TruthManagementStore<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.supports)
    }
}

impl<A: Hash + Eq + PartialEq> PartialEq for TruthManagementStore<A> {
    //FIXME
    fn eq(&self, other: &Self) -> bool {
        self.supports == other.supports
        //true
    }
}

impl<A: Debug + Clone + Merge + Hash + Eq + PartialEq + Add<Output = A>> Add for TruthManagementStore<A> {
    type Output = TruthManagementStore<A>;

    fn add(self, other: Self) -> Self {
        let mut supports = HashSet::new();

        for self_support in self.supports.iter() {
            for other_support in other.supports.iter() {
                let support = (*self_support).clone() + (*other_support).clone();

                supports.insert(support);
            }
        }

        Self {
            system: Rc::clone(&self.system),
            supports
        }
    }
}


impl<A: Debug + Clone + Merge + Hash + Eq + PartialEq + Sub<Output = A>> Sub for TruthManagementStore<A> {
    type Output = TruthManagementStore<A>;

    fn sub(self, other: Self) -> Self {
        let mut supports = HashSet::new();

        for self_support in self.supports.iter() {
            for other_support in other.supports.iter() {
                let support = (*self_support).clone() - (*other_support).clone();
                
                supports.insert(support);
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
            //premises: premises.union(&other_premises).cloned().collect()
        //}
    //}
//}

//impl<A: Div<Output = A>> Div for Supported<A> {
    //type Output = Supported<A>;

    //fn div(self, other: Self) -> Self {
        //Self {
            //value: self.value / other.value,
            //premises: premises.union(&other_premises).cloned().collect()
        //}
    //}
//}


impl<A: Debug + Clone + Hash + Merge + PartialEq + Eq> TruthManagementStore<A> {
    pub fn new(
        tms: &Rc<TruthManagementSystem<TruthManagementStore<A>>>, 
        in_supports: &[(A, &[Premise])]
    ) -> Self {
        let mut supports : HashSet<Supported<A>> = HashSet::new();
        
        // FIXME clone
        for (value, premises) in in_supports {
            supports.insert(Supported::new(value.clone(), premises));
        }

        Self {
            system: Rc::clone(tms),
            supports: supports
        }
    }

    fn from_supports(tms: &Rc<TruthManagementSystem<TruthManagementStore<A>>>, supports: HashSet<Supported<A>>) -> Self {
        Self {
            system: Rc::clone(tms),
            supports
        }
    }

    fn assimilate_many(&self, other_supports: &HashSet<Supported<A>>) -> Self {
        let mut tms = self.clone();

        for other_supported in other_supports.iter() {
            tms = tms.assimilate(other_supported);
        }

        tms
    }

//supported Truth Management Store { supports: [Supported { value: 3.0, premises: {"bar"} }, Supported { value: 5.0, premises: {"baz"} }] }                                                                                                                                                  
//other supported Supported { value: 1.0, premises: {"baz", "foo", "bar"} }                                                                                                                                                                                                                  
//is valid true                                                                                                                                                                                                                                                                              
//any subsumes false  
    fn assimilate(&self, other_supported: &Supported<A>) -> Self {
        let is_valid = self.supports.iter().all(|supported| {
            let premises = supported.premises();
            let other_premises = other_supported.premises();

            if supported.value() == other_supported.value() {
                return true;
            }

            supported.value() != other_supported.value() 
                && !premises.is_subset(&other_premises) && !other_premises.is_subset(&premises)
        });

        // If you can get the same value from any of the current supports
        // while only using a subset of the premises, ie. you require less 
        // information to get to the same answer, then return the current tms
        let any_subsumes = self.supports.iter().any(|supported| {
            supported.subsumes(&other_supported)
        });

        println!("===assimilate===");
        println!("supported {:?}", self);
        println!("other supported {:?}", other_supported);
        println!("is valid {:?}", is_valid);
        println!("any subsumes {:?}", any_subsumes);
        println!("===============");

        if !is_valid || any_subsumes {
            (*self).clone()
        }
        else {

            let subsumed_supports : HashSet<&Supported<A>>= self.supports
                .iter()
                .filter(|supported| other_supported.subsumes(&supported))
                .collect();

            // FIXME: remove cloned
            let mut supports : HashSet<Supported<A>> = HashSet::new();

            for support in self.supports.iter() {
                let mut found = false;

                for &subsumed in subsumed_supports.iter() {
                    if support == subsumed {
                        found = true;
                        break;
                    }
                }

                if !found {
                    supports.insert(support.clone());
                }
            }

            let exists = supports
                .iter()
                .all(|supported| supported == other_supported);

            if !exists {
                supports.insert(other_supported.clone());
            }

            Self::from_supports(&self.system, supports)
        }
    }

    fn strongest_consequence(&self) -> Supported<A> {
        self.supports.iter().fold(None, |acc, instance| {
            match acc {
                None => Some(instance.clone()),
                Some(acc) => {
                    let all_valid = instance.premises().iter().all(|premise| {
                        self.system.premise_is_valid(premise.clone())
                    });

                    if all_valid { Some(acc.merge(&instance)) }
                    else { Some(acc) }
                }
            }
        }).unwrap()
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

    pub fn supports(&self) -> &HashSet<Supported<A>> {
        &self.supports
    }
}




impl<A: Debug + Hash + Eq + Clone + Merge + PartialEq> Merge for TruthManagementStore<A> {
    //fn is_valid(&self, other: &Self) -> bool { true }

    fn merge(&self, other: &Self) -> Self {
        let candidate = self.assimilate_many(&other.supports);
        //if candidate.supports.len() > 2 {
            //return self.clone();
        //}

        let consequence = candidate.strongest_consequence();

        //println!("===merge===");
        //println!("self {:?}", self);
        //println!("other {:?}", other);
        //println!("candidate {:?}", candidate);
        //println!("consequence {:?}", consequence);
        //println!("assimilate {:?}", self.assimilate(&consequence));
        //println!("==========");


        self.assimilate(&consequence)
    }
}
