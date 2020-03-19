
use std::collections::HashSet;

pub enum State<T> {
    Unstable(HashSet<T>),
    Stable(T),
    Failed
}

pub type ID = usize;

/// Min max for quick access
struct Domain {
    min: usize,
    max: usize,
    state: State,
    intervals: Vec<(usize, usize)>
}

impl Domain {
    pub fn new(min: usize, max: usize) -> Self {
        Self {
            min,
            max,
            state: State::Unstable
        }
    }

    pub fn max(&self) -> usize {
        self.max
    }

    pub fn min(&self) -> usize {
        self.min
    }

    pub fn set_max(&mut self, max: usize) -> &State {
        if (self.max > max) { self.max = max; }
        if (self.min > max) { self.state = State::Failed; }

        if (self.max == min) { self.state = State::Stable; }
        else { 
            self.state = State::Unstable; 

            loop {
                match self.intervals.last().unwrap() {
                    &(_, interval_max) if max > interval_max => {
                        self.max = interval_max;
                        break;
                    },
                    &(interval_min, _) if max < interval_min => {
                        self.intervals.pop();
                    } 
                    &(interval_min, _) => {
                        self.max = max;
                        self.intervals.mut_last().unwrap() = (interval_min, max);
                        break;
                    }
                }
            }
        }

        &self.state
    }

    pub fn set_min(&mut self, min: usize) -> &State {
        if (self.min < min) { self.min = min; }
        if (self.max < min) { self.state = State::Failed; }

        if (self.max == min) { 
            self.state = State::Stable; 
        }
        else { 
            self.state = State::Unstable; 

            loop {
                match self.intervals.first().unwrap() {
                    &(interval_min, _) if min < interval_min => {
                        self.min = interval_min;
                        break;
                    },
                    &(_, interval_max) if min > interval_max => {
                        self.intervals.drain(0..0);
                    } 
                    &(_, interval_max) => {
                        self.min = min;
                        self.intervals.mut_first().unwrap() = (min, interval_max);
                        break;
                    }
                }
            }
        }

        return &self.state;
    }

    pub fn remove(&mut self, value: usize) {
        
    }
}
