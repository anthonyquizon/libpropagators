
pub type ID = usize;
pub type Premise = usize;
use std::collections::HashSet;

pub trait Merge {
    fn is_valid(&self, other: &Self) -> bool;
    fn merge(&self, other: &Self) -> Self;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Content<A> {
    Nothing,
    Value(A),
    Contradiction
}

pub struct Cell<A> {
    content: Content<A>,
    dependencies: HashSet<Premise>
}

impl<A: Clone + Merge + PartialEq> Cell<A> {
    pub fn new() -> Self {
        Self {
            content: Content::Nothing,
            dependencies: HashSet::new()
        }
    }

    pub fn assert_ok(&self) {
        assert!(self.content != Content::Contradiction, "Contradiction");
    }

    pub fn is_empty(&self) -> bool {
        match &self.content {
            Content::Nothing => true,
            _ => false
        }
    }

    pub fn wrap(value: A) -> Self {
        let mut cell = Self::new();

        cell.content = Content::Value(value.clone());

        cell
    }

    pub fn to_option(&self) -> Option<A> {
        match &self.content {
            Content::Value(value) => Some(value.clone()),
            _ => None
        }
    }

    pub fn unwrap(&self) -> A {
        match &self.content {
            Content::Value(value) => value.clone(),
            _ => panic!("no value")
        }
    }

    pub fn merge(&self, value: &Self) -> Self {
        let content = match (&self.content, &value.content) {
            (Content::Nothing, _) => value.content.clone(),
            (_, Content::Nothing) => self.content.clone(),
            (Content::Value(a), Content::Value(b)) if a.is_valid(&b) => {
                Content::Value(a.merge(&b))
            },
            (_, _) => Content::Contradiction
        };

        let mut cell = Self::new();

        cell.content = content;

        cell
    }
}
