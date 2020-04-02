
use crate::util::PropagatorID;
use crate::content::Merge;
use std::fmt::Debug;

#[derive(Default, Debug)]
pub struct Cell<T> {
    pub label: String,
    content: T,
    neighbours: Vec<PropagatorID>,
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

impl<T: Merge + PartialEq + Debug> Cell<T> {
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
