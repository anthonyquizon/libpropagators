use std::fmt::Debug;

pub trait Context: Clone + Debug {}

#[derive(Clone, Debug)]
pub struct EmptyContext();

impl Context for EmptyContext {}

