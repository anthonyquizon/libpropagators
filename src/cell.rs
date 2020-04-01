
use crate::util::PropagatorID;

#[derive(Default)]
pub struct Cell<T> {
    pub label: String,
    content: T,
    neighbours: Vec<PropagatorID>,
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
        self.neighbours.push(id);
        self.neighbours.sort_unstable();
        self.neighbours.dedup();
    }

    pub fn read(&self) -> &T {
        &self.content
    }
}

impl<T: Merge + PartialEq> Cell<T> {
    pub fn write(&mut self, content: T) -> Option<&Vec<PropagatorID>> {
            let new_content = self.content.merge(&content);

            if self.content != new_content {
                self.content = new_content;

                Some(&self.neighbours)
            }
            else {
                None
            }
        }
}
