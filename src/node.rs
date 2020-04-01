use std::hash::Hash;
use std::collections::HashSet;

pub struct Node<T, NeighbourID> {
    pub label: String,
    content: T,
    neighbours: HashSet<NeighbourID>,
}

impl<T, NeighbourID: Hash + PartialEq + Eq> Node<T, NeighbourID> {
    pub fn new(content: T) -> Self {
        Self {
            label: String::from(""),
            content: content,
            neighbours: HashSet::new()
        }
    }

    pub fn map<U, F: FnOnce(&T) -> U>(&self, f: F) -> U {
        f(&self.content)
    }

    pub fn add_neighbour(&mut self, id: NeighbourID) {
        self.neighbours.insert(id);
    }

    pub fn for_each_neighbour<F: FnMut(&NeighbourID)>(&self, mut f: F) {
        self.neighbours.iter().for_each(|id| { f(id) })
    }
}

impl<T: PartialEq, NeighbourID: Hash + PartialEq + Eq> Node<T, NeighbourID> {
    pub fn is_content_equal(&self, content: T) -> bool{
        self.content == content
    }
}
