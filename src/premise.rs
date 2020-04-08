use std::fmt::Debug;
use crate::util::CellID;

pub trait Premise: Clone + Ord + PartialOrd + PartialEq + Debug {
    fn make_hypothetical<T: Debug>(value: T, cell_id: CellID) -> Self;
    fn is_hypothetical(&self) -> bool;
}


impl Premise for String {
    fn make_hypothetical<T: Debug>(value: T, cell_id: CellID) -> Self {
        format!("*hypothetical* {:?} {:?}", value, cell_id)
    }

    fn is_hypothetical(&self) -> bool {
        self.contains("*hypothetical*")
    }
}


//impl<T: Clone + Ord + Hash + PartialOrd + PartialEq + Eq + Debug> Premise for PremiseAmb<T> {
    //fn make_hypothetical() -> Self {
        ////uuid
        ////String::from("*hypothetical*")
    //}
//}
