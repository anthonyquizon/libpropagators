
pub type ID = usize;

pub trait Merge {
    fn is_valid(&self, _other: &Self) -> bool { true }
    fn merge(&self, other: &Self) -> Self;
}

#[derive(Debug, PartialEq)]
pub enum Event<A> {
    Changed(A),
    Unchanged,
    Contradiction
}

#[derive(Debug, Clone, PartialEq)]
pub enum Cell<A> {
    Nothing,
    Content(A),
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
            Self::Content(value) => Some(value.clone()),
            _ => None
        }
    }

    pub fn wrap(value: A) -> Self {
        Self::Content(value.clone())
    }

    pub fn unwrap(&self) -> A {
        match self {
            Self::Content(value) => value.clone(),
            _ => panic!("no value")
        }
    }

    pub fn merge(&self, value: &Self) -> Event<Self> {
        match (self, value) {
            (Self::Nothing, _) => Event::Changed(value.clone()),
            (_, Self::Nothing) => Event::Unchanged,
            (Self::Content(a), Self::Content(b)) if a.is_valid(b) => {
                let value = a.merge(b);

                if *a == value {
                    Event::Unchanged
                }
                else {
                    let cell = Self::Content(value);

                    Event::Changed(cell)
                }
            }
            (_, _) => Event::Contradiction
        }
    }
}
