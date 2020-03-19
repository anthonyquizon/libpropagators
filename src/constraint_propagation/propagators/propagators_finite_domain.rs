use propagators::inequality{LessThan};
use variable::{Variable};
use propagator::{Propagator};

struct Equal {
    x: variable::ID,
    y: variable::ID
}

impl Equal {
    pub fn new(x: variable::ID, y: variable::ID) -> Self {
        Self {
            x: x,
            y: y
        }
    }
}

impl Propagator for Equal {
    pub fn propagate() {
    }
}

struct EqualToValue {
}

struct NotEqual {
}
