use std::hash::{Hash};
use std::fmt::Debug;
use crate::util::CellID;

pub trait Premise: Clone + Hash + Ord + PartialOrd + PartialEq + Eq + Debug {
    fn make_hypothetical<T: Debug>(value: T, cell_id: CellID) -> Self;
}

impl Premise for String {
    fn make_hypothetical<T: Debug>(value: T, cell_id: CellID) -> Self {
        format!("*hypothetical* {:?} {:?}", value, cell_id)
    }
}

//#[derive(Clone)]
//pub enum PremiseAmb<T> {
    //Hypothetical(T),
    //Value(T)
//}

//impl<T: Clone + Ord + Hash + PartialOrd + PartialEq + Eq + Debug> Premise for PremiseAmb<T> {
    //fn make_hypothetical() -> Self {
        ////uuid
        ////String::from("*hypothetical*")
    //}
//}
