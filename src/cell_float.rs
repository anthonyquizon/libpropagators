use crate::cell::{ Merge };
use std::ops::{ Add, Sub, Mul, Div };
use ordered_float::NotNan;


#[derive(Hash, Debug, PartialEq, Eq, Clone)]
pub struct Float(NotNan<f64>);

impl Float {
    pub fn new(val: f64) -> Self {
        Self(NotNan::new(val).unwrap())
    }
}

impl Add for Float {
    type Output = Float;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}


impl Sub for Float {
    type Output = Float;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl Merge for Float {
    fn is_valid(&self, value: &Self) -> bool {
        self == value
    }

    fn merge(&self, _other: &Self) -> Self {
        self.clone()
    }
}

