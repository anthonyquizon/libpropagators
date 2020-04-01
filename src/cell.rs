
use crate::util::PropagatorID;
use std::collections::HashSet;

#[derive(Default)]
pub struct Cell<T> {
    pub label: String,
    content: T,
    neighbours: HashSet<PropagatorID>,
}

pub trait Merge {
    fn merge(&self, other: &Self) -> Self;
    fn is_contradiction(&self) -> bool;
}

impl<T: Default> Cell<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> Cell<T> {
    pub fn add_neighbour(&mut self, id: PropagatorID) {
        self.neighbours.insert(id);
    }

    pub fn read(&self) -> &T {
        &self.content
    }
}

impl<T: Merge + PartialEq> Cell<T> {
    pub fn write<F, G>(&mut self, content: T, mut on_changed: F)
        where F: FnMut(&PropagatorID) {
            let new_content = self.content.merge(&content);

            if self.content != new_content {
                self.neighbours.iter().for_each(|id| { 
                    on_changed(id) 
                });

                self.content = new_content
            }
        }
}
