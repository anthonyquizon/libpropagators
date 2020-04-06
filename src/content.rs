pub trait State {
    fn is_empty(&self) -> bool;
    fn is_contradiction(&self) -> bool;
}

pub trait Merge {
    fn merge(&self, other: &Self) -> Self;
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Content<A> {
    Nothing,
    Value(A),
    Contradiction
}

impl<A> Default for Content<A> {
    fn default() -> Self {
        Self::Nothing
    }
}

impl<A> State for Content<A> {
    fn is_empty(&self) -> bool {
        match self {
            Self::Nothing => true,
            _ => false
        }
    }

    fn is_contradiction(&self) -> bool {
        match self {
            Self::Contradiction => true,
            _ => false
        }
    }
}

impl<T> Content<T> {
    pub fn map_or<U, F: FnOnce(&T) -> U>(&self, default: U, f: F) -> U {
        match self {
            Self::Value(val) => f(&val),
            _ => default,
        }
    }
}

impl<T: Clone> Content<T> {
    pub fn map<F: FnOnce(&T) -> Self>(&self, f: F) -> Self {
        match self {
            Self::Value(val) => f(&val),
            _ => self.clone(),
        }
    }

    pub fn lift<F: Fn(&T, &T) -> Self>(&self, other: &Self, f: F) -> Self {
        match (self, other) {
            (Self::Nothing, Self::Nothing) => Self::Nothing,
            (Self::Value(val), Self::Nothing) => Self::Value(val.clone()),
            (Self::Nothing, Self::Value(val)) => Self::Value(val.clone()),
            (Self::Value(a), Self::Value(b)) => f(&a, &b),
            (_, Self::Contradiction) => Self::Contradiction,
            (Self::Contradiction, _) => Self::Contradiction
        }
    }
}
