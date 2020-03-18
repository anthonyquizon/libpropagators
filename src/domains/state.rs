
use std::collections::HashSet;

pub enum State<T> {
    Unstable(HashSet<T>),
    Stable(T),
    Failed
}


