use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Debug, EnumIter, Display)]
pub enum PageTab {
    Device,
    Control,
}

#[derive(Debug)]
pub struct PageTabs {
    pub index: usize,
    pub tabs: Vec<PageTab>,
}

impl PageTabs {
    pub fn new() -> Self {
        Self {
            index: 0,
            tabs: PageTab::iter().collect(),
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.tabs.len()
    }

    pub fn current_tab<'a>(&'a self) -> &'a PageTab {
        &self.tabs[self.index]
    }
}
