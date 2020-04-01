use crate::cell::{ Merge };
use std::ops::{ Add, Sub, Mul, Div };
use ordered_float::NotNan;


#[derive(Hash, Debug, PartialEq, Eq, Clone)]
pub enum Float {
    Nothing,
    Value(NotNan<f64>),
    Contradiction
};

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
    fn merge(&self, other: &Self) -> Self {
        match (&self, other) {
            (Self::Nothing, Self::Nothing) => { Self::Nothing },
            (Self::Value(x), Self::Nothing) => { Self::Value(x) },
            (Self::Nothing, Self::Value(x)) => { Self::Value(x) },
            (Self::Value(x), Self::Value(y)) if x == y => { Self::Value(x) },
            (_, _) => { Self::Contradiction },
        }
    }
}

