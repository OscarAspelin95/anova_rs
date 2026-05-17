use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestrictedVecDeque<T: Sized, const S: usize> {
    pub v: VecDeque<T>,
}

impl<T, const S: usize> Default for RestrictedVecDeque<T, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const S: usize> RestrictedVecDeque<T, S> {
    pub fn new() -> Self {
        const { assert!(S > 0, "Capacity must be at least 1.") }

        Self {
            v: VecDeque::with_capacity(S),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.v.is_empty()
    }

    pub fn capacity(&self) -> usize {
        S
    }

    pub fn sat_push_back(&mut self, v: T) {
        if self.v.len() == S {
            let _ = self.v.pop_front();
        }

        self.v.push_back(v);
    }
}
