use crate::content::{ Content, Merge };
use std::ops::{ Add, Sub, Mul, Div };
use ordered_float::NotNan;
use std::fmt;

pub type Float = Content<NotNan<f64>>;

impl Float {
    pub fn new(val: f64) -> Self {
        Self::Value(NotNan::new(val).unwrap())
    }
}

impl fmt::Debug for Float {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Float: ");

        match self {
            Self::Nothing => write!(f,"Nothing"),
            Self::Value(val) => write!(f, "{:?}", val.into_inner()),
            Self::Contradiction => write!(f, "Contradiction"),
        }
    }
}

impl Add for Float {
    type Output = Float;

    fn add(self, other: Self) -> Self {
        Self::lift(&self, &other, |&a, &b| {
            Self::Value(a + b)
        })
    }
}

impl Sub for Float {
    type Output = Float;

    fn sub(self, other: Self) -> Self {
        Self::lift(&self, &other, |&a, &b| {
            Self::Value(a - b)
        })
    }
}

impl Div for Float {
    type Output = Float;

    fn div(self, other: Self) -> Self {
        Self::lift(&self, &other, |&a, &b| {
            Self::Value(a / b)
        })
    }
}

impl Mul for Float {
    type Output = Float;

    fn mul(self, other: Self) -> Self {
        Self::lift(&self, &other, |&a, &b| {
            Self::Value(a * b)
        })
    }
}

impl Merge for Float {
    fn merge(&self, other: &Self) -> Self {
        Self::lift(self, other, |&a, &b| {
            if a == b { Self::Value(a.clone()) }
            else { Self::Contradiction }
        })
    }
}

