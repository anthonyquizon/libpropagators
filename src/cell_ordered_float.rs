use crate::cell::{ Merge };

use decimal::Decimal;

impl Merge for Decimal {
    fn is_valid(&self, value: &Self) -> bool {
        self == value
    }

    fn merge(&self, other: &Self) -> Self {
        other.clone()
    }
}

