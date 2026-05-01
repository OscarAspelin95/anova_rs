use strum::{Display, EnumIter};

#[derive(Debug, EnumIter, Display)]
pub enum PageTab {
    Device,
    Control,
}
