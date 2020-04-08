use std::hash::{Hash, Hasher};
use crate::content::{ Content, Merge };
use crate::premise::Premise;
use std::ops::{ Add, Sub, Mul, Div };
use std::fmt;
use core::fmt::Debug;
use std::collections::HashSet;

pub type Supported<T, U> = Content<SupportedImpl<T, U>>;

#[derive(Clone, PartialEq, Eq)]
pub struct SupportedImpl<T, U: Premise> {
    value: T,
    premises: HashSet<U>
}

impl<T: Hash, U: Premise + Hash> Hash for SupportedImpl<T, U> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        let mut vec_premises: Vec<&U> = self.premises.iter().collect();

        vec_premises.sort();

        self.value.hash(state);

        for dependency in vec_premises.iter() {
            dependency.hash(state);
        }
    }
}

impl<T: Debug, U: Premise> Debug for Supported<T, U> {
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

impl<T, U: Premise> Supported<T, U> {
    pub fn new(value: T, premises_arr: &[U]) -> Self {
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

    pub fn premises<'a>(&'a self) -> Option<&'a HashSet<U>> {
        match self {
            Self::Value(val) => Some(&val.premises),
            _ => None
        }
    }
}

impl<T: Clone + Debug + Merge + PartialEq, U: Premise> Merge for Supported<T, U> {
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

impl<T: Add<Output=T> + Clone, U: Premise> Add for Supported<T, U> {
    type Output = Supported<T, U>;

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

impl<T: Sub<Output = T> + Clone, U: Premise> Sub for Supported<T, U> {
    type Output = Supported<T, U>;

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

impl<T: Mul<Output = T> + Clone, U: Premise> Mul for Supported<T, U> {
    type Output = Supported<T, U>;

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

impl<T: Div<Output = T> + Clone, U: Premise> Div for Supported<T, U> {
    type Output = Supported<T, U>;

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

impl<T: Merge + PartialEq, U: Premise> Supported<T, U> {
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
