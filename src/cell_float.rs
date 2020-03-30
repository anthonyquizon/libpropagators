use crate::cell::{ Merge };


#[derive(PartialEq, Clone)]
pub struct Float(ordered_float::NotNan<f64>);

impl Merge for Float {
    fn is_valid(&self, value: &Self) -> bool {
        self == value
    }

    fn merge(&self, other: &Self) -> Self {
        self.clone()
    }
}

