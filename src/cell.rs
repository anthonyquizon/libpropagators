
use crate::node::Node;
use crate::util::PropagatorID;

pub type Cell<T> = Node<T, PropagatorID>;

pub trait Merge {
    fn merge(&self, other: &Self) -> Self;
    fn is_contradiction(&self) -> bool;
}

impl<T: Merge + PartialEq> Cell<T> {
    pub fn write<F, G>(&self, content: T, on_changed: F, on_contradiction: G) -> Self 
        where F: FnMut(&PropagatorID),
              G: FnOnce() {
                  let new_content = self.map(|self_content| {
                      self_content.merge(&content)
                  });

                  if new_content.is_contradiction() {
                      on_contradiction();
                  }

                  if !self.is_content_equal(new_content) {
                    self.for_each_neighbour(on_changed)
                  }

                  Self::new(content)
              }

    //pub fn read(&self) -> T {
    //}
}
