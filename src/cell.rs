
pub type ID = usize;

pub trait Merge {
    fn is_valid(&self, _other: &Self) -> bool { true }
    fn merge(&self, other: &Self) -> Self;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Cell<A> {
    Nothing,
    Value(A),
    Contradiction
}

impl<A: Clone + Merge + PartialEq> Cell<A> {
    pub fn new() -> Self {
        Self::Nothing
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Nothing => true,
            _ => false
        }
    }

    pub fn to_option(&self) -> Option<A> {
        match &self {
            Self::Value(value) => Some(value.clone()),
            _ => None
        }
    }

    pub fn assert_ok(&self) {
        match self {
            Self::Contradiction => panic!("Contradication"),
            _ => {}
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
