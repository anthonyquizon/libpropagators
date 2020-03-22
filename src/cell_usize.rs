use crate::cell::{ Merge };

impl Merge for usize {
    fn is_valid(&self, value: &Self) -> bool {
        self == value
    }

    fn merge(&self, other: &Self) -> Self {
        other.clone()
    }
}

//impl Bool for usize {
    //fn to_bool(self) -> bool {
        //self != 0
    //}
//}

