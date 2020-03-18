
pub type ID = usize;

trait Propagator {
    pub fn new() -> Self;
    pub fn propagate();
}

