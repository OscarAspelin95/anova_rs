use serde::Serialize;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, EnumIter, Serialize)]
pub enum ControlType {
    Start,
    Set,
    Stop,
}

#[derive(Debug, Clone, Serialize)]
pub struct Control {
    current_index: Option<usize>,
    next_index: Option<usize>,
    controllers: Vec<ControlType>,
}

/// We have this pattern here, in device and in tabs.
/// Should check if we can make a generic interface/trait.
impl Control {
    pub fn new() -> Self {
        Self {
            current_index: None,
            next_index: None,
            controllers: ControlType::iter().collect(),
        }
    }

    pub fn next_control(&mut self) {
        let next_index = match self.next_index {
            None => return,
            Some(next_index) => next_index,
        };

        self.next_index = Some((next_index + 1).min(self.controllers.len() - 1))
    }

    pub fn previous_control(&mut self) {
        let next_index = match self.next_index {
            None => return,
            Some(next_index) => next_index,
        };

        self.next_index = Some(next_index.saturating_sub(1));
    }

    pub fn update_control(&mut self) {
        match (self.current_index, self.next_index) {
            // no currently chosen control
            (None, Some(next_index)) => self.current_index = Some(next_index),

            // check if update or unset.
            (Some(current_index), Some(next_index)) => match current_index == next_index {
                true => self.current_index = None,
                false => self.current_index = Some(next_index),
            },
            _ => {}
        }
    }

    pub fn current_control<'a>(&'a self) -> Option<&'a ControlType> {
        match self.current_index {
            None => None,
            Some(current_index) => self.controllers.get(current_index),
        }
    }
}
