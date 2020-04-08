use std::fmt::Debug;

pub trait Context: Clone + Debug {
    type Action;

    fn run_action(&mut self, action: Self::Action);
}

#[derive(Clone, Debug)]
pub struct EmptyContext();

impl Context for EmptyContext {
    type Action=();

    fn run_action(&mut self, _: Self::Action) {}
}

