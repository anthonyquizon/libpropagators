use std::rc::{Rc};
use std::fmt::{Debug, Formatter, Result};
use crate::content::{Content, Merge};
use crate::content_supported::Supported;
use std::hash::Hash;
use std::ops::{ Add, Sub, Mul, Div };
use crate::tms::TruthManagementSystem;
use crate::premise::Premise;
use std::collections::HashSet;

pub type TruthManagementStore<T, Premise> = Content<TruthManagementStoreImpl<T, Premise>>;

#[derive(Clone)]
pub struct TruthManagementStoreImpl<T, U: Premise> {
    system: Rc<TruthManagementSystem<TruthManagementStore<T, U>, U>>,
    supports: HashSet<Supported<T, U>>,
}

impl<T: Debug + Hash + Eq + PartialEq, U: Premise> Debug for TruthManagementStore< T, U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f,"TruthManagementStore::");

        match self {
            Self::Nothing => write!(f,"Nothing"),
            Self::Value(val) => {
                write!(f, "{:?}", val.supports);
                Ok(())
            },
            Self::Contradiction => write!(f, "Contradiction"),
        }
    }
}

impl<T: Debug + Hash + Eq + PartialEq, U: Premise> PartialEq for TruthManagementStore<T, U> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nothing, Self::Nothing) => true,
            (Self::Value(a), Self::Value(b)) => {
                a.supports == b.supports
            }
            _ => false,
        }
    }
}

impl<T: Debug + Clone + Merge + PartialEq + Eq + Hash, U: Premise> TruthManagementStore<T, U> {
    pub fn new(
        tms: &Rc<TruthManagementSystem<TruthManagementStore<T, U>, U>>, 
        in_supports: &[(T, &[U])]
    ) -> Self {
        let mut supports : HashSet<Supported<T, U>> = HashSet::new();
        
        // FIXME remove clone
        for (value, premises) in in_supports {
            supports.insert(Supported::new(value.clone(), premises));
        }

        let tms = TruthManagementStoreImpl {
            system: Rc::clone(tms),
            supports
        };

        Self::Value(tms)
    }

    fn assimilate(&self, other: &Self) -> Self {
        Self::lift(self, other, |a, b| {
            let mut tms = a.clone();

            for other_supported in b.supports.iter() {
                // If you can get the same value from any of the current supports
                // while only using a subset of the premises, ie. you require less 
                // information to get to the same answer, then return the current tms
                let any_subsumes = a.supports.iter().any(|supported| {
                    supported.subsumes(&other_supported)
                });

                if !any_subsumes {
                    // FIXME: remove cloned
                    let mut supports : HashSet<Supported<T, U>>= a.supports
                        .iter()
                        .cloned()
                        //NB: the subsumes objects are swapped compared to the any_subsumes clause
                        .filter(|supported| !other_supported.subsumes(&supported))
                        .collect();

                    supports.insert(other_supported.clone());

                    //FIXME remove cloned
                    tms.supports = tms.supports.union(&supports).cloned().collect();
                }
            }

            Self::Value(tms)
        })
    }

    fn strongest_consequence(&self) -> Self {
        Self::map(self, |tms| {
            //FIXME: remove clone
            let mut tms = tms.clone();
            let supports = tms.supports.iter().fold(None, |acc, instance| {
                match acc {
                    None => Some(instance.clone()),
                    Some(acc) => {
                        let all_valid = instance.premises().map_or(false, |premises| {
                            premises.iter().all(|premise| {
                                tms.system.premise_in(premise)
                            })
                        });

                        if all_valid { Some(acc.merge(&instance)) }
                        else { Some(acc) }
                    }
                }
            }).unwrap();

            tms.supports = HashSet::new();
            tms.supports.insert(supports);

            Self::Value(tms)
        })
    }

    //pub fn check_consistent(&self) -> bool {
        //match 
    //}

    //pub fn query(&mut self) -> T {
        //let answer = self.strongest_consequence();
        //let better_tms = self.assimilate(&answer);

        //if *self != better_tms { 
            //self.supports = better_tms.supports;
        //}

        //// FIXME remove clone
        //(*answer.value()).clone()
    //}

    //pub fn supports(&self) -> &HashSet<Supported<T, U>> {
        //&self.supports
    //}
}
impl<T: Debug + Clone + Merge + PartialEq + Eq + Hash, U: Premise> Merge for TruthManagementStore<T, U> {
    fn merge(&self, other: &Self) -> Self {
        let candidate = self.assimilate(&other);
        let consequence = candidate.strongest_consequence();

        //TODO check if contradiction

        candidate.assimilate(&consequence)
    }
}

impl<T: Hash + PartialEq + Eq + Clone + Add<Output = T>, U: Premise + Clone> Add for TruthManagementStore<T, U> {
    type Output = TruthManagementStore<T, U>;

    fn add(self, other: Self) -> Self {
        Self::lift(&self, &other, |a, b| {
            let mut supports = HashSet::new();

            for self_support in a.supports.iter() {
                for other_support in b.supports.iter() {
                    let support = (*self_support).clone() + (*other_support).clone();

                    supports.insert(support);
                }
            }

            let tms = TruthManagementStoreImpl {
                system: Rc::clone(&a.system),
                supports
            };

            Self::Value(tms)
        })
    }
}

impl<T: Debug + Clone + Merge + Hash + Eq + PartialEq + Sub<Output = T>, U: Premise> Sub for TruthManagementStore<T, U> {
    type Output = TruthManagementStore<T, U>;

    fn sub(self, other: Self) -> Self {
        Self::lift(&self, &other, |a, b| {
            let mut supports = HashSet::new();

            for self_support in a.supports.iter() {
                for other_support in b.supports.iter() {
                    let support = (*self_support).clone() - (*other_support).clone();

                    supports.insert(support);
                }
            }

            let tms = TruthManagementStoreImpl {
                system: Rc::clone(&a.system),
                supports
            };

            Self::Value(tms)
        })
    }
}

////impl<A: Mul<Output = A>> Mul for Supported<A> {
    ////type Output = Supported<A>;

    ////fn mul(self, other: Self) -> Self {
        ////Self {
            ////value: self.value * other.value,
            ////premises: premises.union(&other_premises).cloned().collect()
        ////}
    ////}
////}

////impl<A: Div<Output = A>> Div for Supported<A> {
    ////type Output = Supported<A>;

    ////fn div(self, other: Self) -> Self {
        ////Self {
            ////value: self.value / other.value,
            ////premises: premises.union(&other_premises).cloned().collect()
        ////}
    ////}
////}


//impl<A: Debug + Clone + Hash + Merge + PartialEq + Eq> TruthManagementStore<A> {
    //pub fn new(
        //tms: &Rc<TruthManagementSystem<TruthManagementStore<A>>>, 
        //in_supports: &[(A, &[Premise])]
    //) -> Self {
        //let mut supports : HashSet<Supported<A>> = HashSet::new();
        
        //// FIXME clone
        //for (value, premises) in in_supports {
            //supports.insert(Supported::new(value.clone(), premises));
        //}

        //Self {
            //system: Rc::clone(tms),
            //supports: supports
        //}
    //}


//}


