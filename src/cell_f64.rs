use crate::cell::{ Merge };

impl Merge for f64 {
    fn is_valid(&self, value: &Self) -> bool {
        self == value
    }

    fn merge(&self, other: &Self) -> Self {
        other.clone()
    }
}

