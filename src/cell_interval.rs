use crate::cell::{ Merge };

use std::cmp;
use std::ops::{ Add, Sub, Mul, Div };

macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = max!($($z),*);
        if $x > y {
            $x
        } else {
            y
        }
    }}
}

macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = min!($($z),*);
        if $x < y {
            $x
        } else {
            y
        }
    }}
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Interval {
    min: i32,
    max: i32
}

impl Add for Interval {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let min = self.min + other.min;
        let max = self.max + other.max;

        Self {
            min, 
            max
        }
    }
}

impl Sub for Interval {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let min = self.min - other.max;
        let max = self.max - other.min;

        Self {
            min, 
            max
        }
    }
}

impl Mul for Interval {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let p1 = self.min * other.min;
        let p2 = self.min * other.max;
        let p3 = self.max * other.min;
        let p4 = self.max * other.max;

        let min = min!(p1, p2, p3, p4);
        let max = max!(p1, p2, p3, p4);

        Self {
            min, 
            max
        }
    }
}

impl Div for Interval {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if other.min <= 0 && 0 <= other.max && other.min <= other.max {
            panic!("cannot divide by interval spanning 0");
        }

        let other_inverse = Self { 
            min: 1 / other.max,  
            max: 1 / other.min
        };

        self * other_inverse
    }
}


impl Merge for Interval {
    fn is_valid(&self, _other: &Self) -> bool { true }

    fn merge(&self, other: &Self) -> Self {
        let min = cmp::max(self.min, other.min);
        let max = cmp::min(self.max, other.max);

        Self {
            min,
            max
        }
    }
}

impl Interval {
    pub fn new(min: i32, max: i32) -> Self {
        Self {
            min,
            max
        }
    }
}

