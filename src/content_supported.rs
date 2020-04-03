use std::hash::{Hash, Hasher};
use crate::content::{ Content, Merge };
use crate::tms::Premise;
use std::ops::{ Add, Sub, Mul, Div };
use std::fmt;
use core::fmt::Debug;
use std::collections::HashSet;


pub type Supported<T> = Content<SupportedImpl<T>>;

#[derive(Clone, Debug, PartialEq)]
pub struct SupportedImpl<T> {
    value: T,
    premises: HashSet<Premise>
}

impl<T: Debug> Debug for Supported<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"Supported::");

        match self {
            Self::Nothing => write!(f,"Nothing"),
            Self::Value(val) => {
                write!(f, "Value: {:?}", val.value);

                write!(f, "Premises: ");

                val.premises.iter().for_each(|premise| {
                    write!(f, "{:?}", premise);
                });

                Ok(())
            },
            Self::Contradiction => write!(f, "Contradiction"),
        }
    }
}

impl<T> Supported<T> {
    pub fn new(value: T, premises_arr: &[Premise]) -> Self {
        let mut premises = HashSet::new();

        for premise in premises_arr {
            premises.insert(premise.clone());
        }

        let value = SupportedImpl {
            value,
            premises
        };

        Self::Value(value)
    }
}

impl<T: Clone + Debug + Merge + PartialEq> Merge for Supported<T> {
    fn merge(&self, other: &Self) -> Self {
        Self::lift(self, other, |a, b| {
            let merged_value = a.value.merge(&b.value);

            if merged_value == a.value {
                if b.value.merge(&merged_value) == b.value {
                    if a.premises != b.premises && 
                        b.premises.is_subset(&a.premises) 
                    {
                        Self::Value((*b).clone())
                    } else {
                        Self::Value((*a).clone())
                    }
                }
                else {
                    Self::Value((*a).clone())
                }
            }
            else if merged_value == b.value {
                Self::Value((*b).clone())
            }
            else {
                let value = SupportedImpl {
                    value: merged_value,
                    premises: a.premises.union(&b.premises).cloned().collect()
                };

                Self::Value(value)
            }
        })
    }
}

impl<T: Add<Output=T> + Clone> Add for Supported<T> {
    type Output = Supported<T>;

    fn add(self, other: Self) -> Self::Output {
        Self::lift(&self, &other, |a, b| {
            let value = SupportedImpl {
                //FIXME remove clone
                value: a.value.clone() + b.value.clone(),
                premises: a.premises.union(&b.premises).cloned().collect()
            };

            Self::Value(value)
        })
    }
}

impl<T: Sub<Output = T> + Clone> Sub for Supported<T> {
    type Output = Supported<T>;

    fn sub(self, other: Self) -> Self {
        Self::lift(&self, &other, |a, b| {
            let value = SupportedImpl {
                //FIXME remove clone 
                value: a.value.clone() - b.value.clone(),
                premises: a.premises.union(&b.premises).cloned().collect()
            };

            Self::Value(value)
        })
    }
}

impl<T: Mul<Output = T> + Clone> Mul for Supported<T> {
    type Output = Supported<T>;

    fn mul(self, other: Self) -> Self {
        Self::lift(&self, &other, |a, b| {
            let value = SupportedImpl {
                //FIXME remove clone 
                value: a.value.clone() * b.value.clone(),
                premises: a.premises.union(&b.premises).cloned().collect()
            };

            Self::Value(value)
        })
    }
}

impl<T: Div<Output = T> + Clone> Div for Supported<T> {
    type Output = Supported<T>;

    fn div(self, other: Self) -> Self {
        Self::lift(&self, &other, |a, b| {
            let value = SupportedImpl {
                //FIXME remove clone 
                value: a.value.clone() / b.value.clone(),
                premises: a.premises.union(&b.premises).cloned().collect()
            };

            Self::Value(value)
        })
    }
}

impl<T: Merge + PartialEq> Supported<T> {
    pub fn subsumes(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nothing, Self::Nothing) => true,
            (Self::Value(_), Self::Nothing) => true,
            (Self::Value(a), Self::Value(b)) => {
                a.value == a.value.merge(&b.value) && 
                    a.premises.is_subset(&b.premises)
            }
            _ => false
        }
    }
}
