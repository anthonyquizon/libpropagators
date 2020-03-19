
pub type ID = usize;

pub trait Merge {
    fn is_valid(&self, other: &Self) -> bool;
    fn merge(&self, other: &Self) -> Self;
}

#[derive(Debug, Clone)]
pub enum Cell<A> {
    Nothing,
    Value(A),
    Contradiction
}

impl<A: Clone + Merge> Cell<A> {
    pub fn new() -> Self {
        Self::Nothing
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Nothing => true,
            _ => false
        }
    }

    pub fn wrap(value: A) -> Self {
        Self::Value(value.clone())
    }

    pub fn unwrap(&self) -> A {
        match self {
            Self::Value(value) => value.clone(),
            _ => panic!("no value")
        }
    }

    pub fn merge(&self, value: &Self) -> Self {
        match (self, value) {
            (Self::Nothing, _) => value.clone(),
            (_, Self::Nothing) => self.clone(),
            (Self::Value(a), Self::Value(b)) if a.is_valid(b) => {
                Self::Value(a.merge(b))
            }
            (_, _) => Self::Contradiction
        }
    }
}
