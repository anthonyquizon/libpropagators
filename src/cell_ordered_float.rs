use crate::cell::{ Merge };


#[derive(PartialEq, Clone)]
pub struct OrderedFloat(ordered_float::OrderedFloat<f64>);

impl Merge for OrderedFloat {
    fn is_valid(&self, value: &Self) -> bool {
        self == value
    }

    fn merge(&self, other: &Self) -> Self {
        other.clone()
    }
}

