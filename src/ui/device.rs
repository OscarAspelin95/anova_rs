use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{List, ListItem};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::app::App;

impl App {
    /// Break this into better logic.
    pub fn render_device_page(&self, area: Rect, buf: &mut Buffer) {
        // split layout
        let [list_area, help_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);

        // show devices
        let items: Vec<ListItem> = self
            .anova_devices
            .devices
            .iter()
            .enumerate()
            .map(|(i, d)| {
                // default
                let mut style = Style::default().fg(Color::Cyan);

                // highlighted
                if Some(i) == self.anova_devices.next_index {
                    style = Style::default().fg(Color::Yellow).bold();
                }

                // active
                let t = match Some(i) == self.anova_devices.current_index {
                    true => "● ",
                    false => "○ ",
                };

                ListItem::new(format!(
                    "{}{} | {} | {} | {}",
                    t, d.cooker_id, d.name, d.r#type, d.paired_at
                ))
                .style(style)
            })
            .collect();

        if !items.is_empty() {
            List::new(items)
                .block(
                    Block::bordered()
                        .border_type(BorderType::Rounded)
                        .title("Devices"),
                )
                .highlight_symbol("> ")
                .render(list_area, buf);
        } else {
            Paragraph::new("No devices found (yet)")
                .alignment(Alignment::Center)
                .red()
                .render(list_area, buf);
        }

        // show help text
        let divider = " | ".dark_gray();

        Paragraph::new(Line::from(vec![
            "↑/↓".magenta(),
            " navigate ".into(),
            divider.clone(),
            "↵".magenta(),
            " select ".into(),
            divider.clone(),
            "↹ ".magenta(),
            " change view ".into(),
        ]))
        .alignment(Alignment::Center)
        .render(help_area, buf);
    }
}
