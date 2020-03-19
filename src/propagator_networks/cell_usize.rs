use crate::propagator_networks::cell::{ Merge };

impl Merge for usize {
    fn is_valid(&self, value: &Self) -> bool {
        self == value
    }

    fn merge(&self, value: &Self) -> Self {
        value.clone()
    }
}

//impl Bool for usize {
    //fn to_bool(self) -> bool {
        //self != 0
    //}
//}

