enum State {
    Unstable,
    Stable,
    Failed
}

struct Domain {
    min: usize,
    max: usize,
    state: State
}

impl Domain {
    pub fn new(min: usize, max: usize) -> Self {
        Self {
            min,
            max,
            state: State::Unstable
        }
    }

    pub fn adj_max(&mut self, max: usize) {
        if (self.max > max) { self.max = max; }
        if (self.min > max) { self.state = State::Failed; }

        //TODO prune domain
    }

    pub fn adj_min(&mut self, min: usize) {
        if (self.min < min) { self.min = min; }
        if (self.max < min) { self.state = State::Failed; }

        //TODO prune domain
    }
}
