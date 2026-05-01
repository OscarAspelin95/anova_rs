use serde::Serialize;
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Serialize)]
pub struct FixedValueSet<T> {
    // set by the user.
    index: Option<usize>,
    // currently highlighted.
    selected: Option<usize>,
    values: Vec<T>,
}

impl<T: IntoEnumIterator> FixedValueSet<T> {
    pub fn new(index: Option<usize>, selected: Option<usize>) -> Self {
        Self {
            index: index,
            selected: selected,
            values: T::iter().collect(),
        }
    }

    pub fn new_empty() -> Self {
        Self {
            index: None,
            selected: None,
            values: T::iter().collect(),
        }
    }

    pub fn values(&self) -> &[T] {
        &self.values
    }

    pub fn index(&self) -> Option<usize> {
        self.index
    }

    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    /// Wrap around increment for selected.
    pub fn increment(&mut self) {
        let selected = match self.selected {
            None => return,
            Some(selected) => selected,
        };

        self.selected = Some((selected + 1) % self.values.len())
    }

    /// Wrap around decrement for selected.
    pub fn decrement(&mut self) {
        let selected = match self.selected {
            None => return,
            Some(selected) => selected,
        };

        self.selected = Some((selected + self.values.len() - 1) % self.values.len());
    }

    /// Sets `index` = `selected`.
    pub fn set(&mut self) {
        match (self.index, self.selected) {
            // nothing set, but something selected
            (None, Some(selected)) => self.index = Some(selected),

            // something set, something selected
            (Some(index), Some(selected)) => match index == selected {
                true => self.index = None,            // deselect.
                false => self.index = Some(selected), // index -> selected.
            },
            _ => {}
        }
    }

    /// Sets `index` as the `selected` + 1 .
    pub fn increment_set(&mut self) {
        self.increment();
        self.set();
    }

    /// Current value.
    pub fn current<'a>(&'a self) -> Option<&'a T> {
        match self.index {
            None => None,
            Some(current_index) => self.values.get(current_index),
        }
    }
}
