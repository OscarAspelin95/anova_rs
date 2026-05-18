use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

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

    pub fn len(&self) -> usize {
        self.v.len()
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

#[inline]
pub fn round_with_margin(t: f64, margin: f64) -> f64 {
    (t * margin).ceil()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[test]
    fn test_sat_push_back() {
        let mut values: RestrictedVecDeque<f64, 3> = RestrictedVecDeque::new();

        assert!(values.is_empty());
        assert_eq!(values.capacity(), 3);

        values.sat_push_back(1.0);
        assert!(!values.is_empty());
        values.sat_push_back(2.0);
        values.sat_push_back(3.0);

        // [1.0, 2.0, 3.0]
        assert_eq!(values.len(), 3);

        // [2.0, 3.0, 4.0]
        values.sat_push_back(4.0);
        assert_eq!(values.len(), 3);

        let vec = values.v.iter().collect::<Vec<_>>();
        assert!(matches!(&vec[..], &[2.0, 3.0, 4.0]));
    }

    #[rstest]
    #[case(30.0, 1.1, 33.0)]
    #[case(35.0, 1.2, 42.0)]
    #[case(41.0, 1.1, 46.0)]
    fn test_round_with_margin(#[case] input: f64, #[case] margin: f64, #[case] expected: f64) {
        assert_eq!(round_with_margin(input, margin), expected);
    }
}
