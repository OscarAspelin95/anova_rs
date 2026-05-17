//! we can potentially put a .to_component() method
//! on ApcState to simplify some of the component generation.
mod control;
mod device;
mod plot;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::Style;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::app::App;

use crate::types::PageTab;
use ratatui::widgets::Tabs;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Fill(1)])
            .split(area);

        let tabs = Tabs::new(
            self.page_tabs
                .values()
                .iter()
                .map(|t: &PageTab| t.to_string()),
        )
        .select(self.page_tabs.selected().expect("must be selected"))
        .block(Block::bordered().title("Anova UI"))
        .highlight_style(Style::default().fg(Color::Cyan).bold())
        .divider("|");

        tabs.render(chunks[0], buf);

        match self.page_tabs.current() {
            Some(&PageTab::Device) => self.render_device_page(chunks[1], buf),
            Some(&PageTab::Control) => self.render_control_page(chunks[1], buf),
            Some(&PageTab::Plot) => self.render_plot_page(chunks[1], buf),
            _ => {}
        }
    }
}

impl App {
    fn render_control_no_device_connected(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("No device connected")
            .block(Block::bordered().border_type(BorderType::Rounded))
            .fg(Color::DarkGray)
            .centered()
            .render(area, buf);
    }
}
