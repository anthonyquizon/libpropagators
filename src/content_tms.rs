use std::rc::{Rc};
use std::fmt::{Debug, Formatter, Result};
use crate::content::{Content, Merge};
use crate::content_supported::Supported;
use std::hash::{Hash};
use std::ops::{ Add, Sub, Mul, Div };
use crate::tms::{ TruthManagementSystem, Premise };
use std::collections::HashSet;

pub type TruthManagementStore<T, Premise> = Content<TruthManagementStoreImpl<T, Premise>>;

#[derive(Clone)]
pub struct TruthManagementStoreImpl<T, U: Premise> {
    system: Rc<TruthManagementSystem<TruthManagementStore<T, U>, U>>,
    supports: HashSet<Supported<T, U>>,
}

impl<T: Debug + Hash + Eq + PartialEq, U: Premise> Debug for TruthManagementStore<T, U> {
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


//impl<A: Debug + Clone + Merge + Hash + Eq + PartialEq + Sub<Output = A>> Sub for TruthManagementStore<A> {
    //type Output = TruthManagementStore<A>;

    //fn sub(self, other: Self) -> Self {
        //let mut supports = HashSet::new();

        //for self_support in self.supports.iter() {
            //for other_support in other.supports.iter() {
                //let support = (*self_support).clone() - (*other_support).clone();
                
                //supports.insert(support);
            //}
        //}

        //Self {
            //system: Rc::clone(&self.system),
            //supports
        //}
    //}
//}

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

    //fn from_supports(tms: &Rc<TruthManagementSystem<TruthManagementStore<A>>>, supports: HashSet<Supported<A>>) -> Self {
        //Self {
            //system: Rc::clone(tms),
            //supports
        //}
    //}

    //fn assimilate_many(&self, other_supports: &HashSet<Supported<A>>) -> Self {
        //let mut tms = self.clone();

        //for other_supported in other_supports.iter() {
            //tms = tms.assimilate(other_supported);
        //}

        //tms
    //}

    //fn assimilate(&self, other_supported: &Supported<A>) -> Self {
        //// If you can get the same value from any of the current supports
        //// while only using a subset of the premises, ie. you require less 
        //// information to get to the same answer, then return the current tms
        //let any_subsumes = self.supports.iter().any(|supported| {
            //supported.subsumes(&other_supported)
        //});

        //if any_subsumes {
            //(*self).clone()
        //}
        //else {
            //// FIXME: remove cloned
            //let mut supports : HashSet<Supported<A>>= self.supports
                //.iter()
                //.cloned()
                ////NB: the subsumes objects are swapped compared to the any_subsumes clause
                //.filter(|supported| !other_supported.subsumes(&supported))
                //.collect();

            //let exists = supports
                //.iter()
                //.all(|supported| supported == other_supported);

            //if !exists {
                //supports.insert(other_supported.clone());
            //}

            //Self::from_supports(&self.system, supports)
        //}
    //}

    //fn strongest_consequence(&self) -> Supported<A> {
        //self.supports.iter().fold(None, |acc, instance| {
            //match acc {
                //None => Some(instance.clone()),
                //Some(acc) => {
                    //let all_valid = instance.premises().iter().all(|premise| {
                        //self.system.premise_is_valid(premise.clone())
                    //});

                    //if all_valid { Some(acc.merge(&instance)) }
                    //else { Some(acc) }
                //}
            //}
        //}).unwrap()
    //}

    //pub fn query(&mut self) -> A {
        //let answer = self.strongest_consequence();
        //let better_tms = self.assimilate(&answer);

        //if *self != better_tms { 
            //self.supports = better_tms.supports;
        //}

        //// FIXME remove clone
        //(*answer.value()).clone()
    //}

    //pub fn supports(&self) -> &HashSet<Supported<A>> {
        //&self.supports
    //}
//}


//impl<T: Debug + Clone + Merge + PartialEq + Eq + Hash, Premise> Merge for TruthManagementStore<T, Premise> {
    ////TODO merge  
    //fn merge(&self, other: &Self) -> Self {
        //let candidate = self.assimilate_many(&other.supports);
        //let consequence = candidate.strongest_consequence();

        ////TODO check if contradiction

        //candidate.assimilate(&consequence)
    //}
//}
