use crate::content::{ Content, Merge };
use std::ops::{ Add, Sub, Mul, Div };
use ordered_float::NotNan;

pub type Float = Content<NotNan<f64>>;

impl Float {
    pub fn new(val: f64) -> Self {
        Self::Value(NotNan::new(val).unwrap())
    }
}

    //pub fn map<F: Fn() -> Self>(self, other: Self,) -> Self {
        //match (self, other) {
            //(Self::Nothing, Self::Nothing) => Self::Nothing,
            //(Self::Value(val), Self::Nothing) => Self::Value(val.clone()),
            //(Self::Nothing, Self::Value(val)) => Self::Value(val.clone()),
            //(Self::Value, Self::Value(val)) => Self::Value(val.clone()),
        //}
    //}
//}

//impl Add for Float {
    //type Output = Float;

    //fn add(self, other: Self) -> Self {
        //match 
        //Self(self.0 + other.0)
    //}
//}

//impl Sub for Float {
    //type Output = Float;

    //fn sub(self, other: Self) -> Self {
        //Self(self.0 - other.0)
    //}
//}

impl Merge for Float {
    fn merge(&self, other: &Self) -> Self {
        Self::Nothing
        //match (&self, other) {
            //(Self::Nothing, Self::Nothing) => { Self::Nothing },
            //(Self::Value(x), Self::Nothing) => { Self::Value(x) },
            //(Self::Nothing, Self::Value(x)) => { Self::Value(x) },
            //(Self::Value(x), Self::Value(y)) if x == y => { Self::Value(x) },
            //(_, _) => { Self::Contradiction },
        //}
    }
}

