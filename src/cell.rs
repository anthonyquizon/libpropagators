
use std::fmt::Debug;

pub type ID = usize;

pub trait Merge {
    fn is_valid(&self, _other: &Self) -> bool { true }
    fn merge(&self, other: &Self) -> Self;
}

#[derive(Debug, PartialEq)]
pub enum Event {
    Changed,
    Unchanged,
    Contradiction
}

#[derive(Debug, Clone, PartialEq)]
pub enum Content<A> {
    Nothing,
    Content(A),
    Contradiction
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cell<A> {
    label: String,
    content: Content<A>
}

impl<A: Debug + Clone + Merge + PartialEq> Cell<A> {
    pub fn new() -> Self {
        Self {
            label: String::from(""),
            content: Content::Nothing
        }
    }

    pub fn set_label(&mut self, label: &str) {
        self.label = String::from(label);
    }

    pub fn is_empty(&self) -> bool {
        match &self.content {
            Content::Nothing => true,
            _ => false
        }
    }

    pub fn to_option(&self) -> Option<A> {
        match &self.content {
            Content::Content(value) => Some(value.clone()),
            _ => None
        }
    }

    pub fn wrap(value: A) -> Self {
        let mut cell = Self::new();
        cell.content = Content::Content(value.clone());
        cell
    }

    pub fn unwrap(&self) -> A {
        match &self.content {
            Content::Content(value) => value.clone(),
            _ => panic!("no value")
        }
    }

    pub fn merge(&mut self, value: &A) -> Event {
        match &self.content {
            Content::Nothing => {
                self.content = Content::Content(value.clone());

                Event::Changed
            },
            Content::Content(old_value) if old_value.is_valid(value) => {
                let merged_value = old_value.merge(value);

                if *old_value == merged_value {
                    Event::Unchanged
                }
                else {
                    self.content = Content::Content(merged_value);

                    Event::Changed
                }
            }
            _ => {
                self.content = Content::Contradiction;
                Event::Contradiction
            }
        }
    }
}
