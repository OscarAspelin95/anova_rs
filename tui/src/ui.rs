use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::Style;
use ratatui::text::{Line, Span, Text};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::app::App;

use crate::types::{Devices, PageTab, PageTabs};
use ratatui::widgets::{List, ListItem, Tabs};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Fill(1)])
            .split(area);

        let tabs = Tabs::new(self.page_tabs.tabs.iter().map(|t| t.to_string()))
            .select(self.page_tabs.index)
            .block(Block::bordered().title("Anova UI"))
            .highlight_style(Style::default().fg(Color::Cyan).bold())
            .divider("|");

        tabs.render(chunks[0], buf);

        match self.page_tabs.current_tab() {
            &PageTab::Device => self.render_device_page(chunks[1], buf),
            &PageTab::Control => self.render_control_page(chunks[1], buf),
        }
    }
}

impl App {
    fn render_device_page(&self, area: Rect, buf: &mut Buffer) {
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
                    true => "*",
                    false => "",
                };

                ListItem::new(format!(
                    "{}{} | {} | {} | {}",
                    t, d.cooker_id, d.name, d.r#type, d.paired_at
                ))
                .style(style)
            })
            .collect();

        if items.len() > 0 {
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

    fn render_control_page(&self, area: Rect, buf: &mut Buffer) {
        let [control_area, help_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);

        // render control
        let text = match &self.anova_devices.current_device() {
            Some(device) => {
                let header = Line::from(format!(
                    "{} | {} | {}",
                    device.cooker_id, device.name, device.r#type
                ));

                let mut lines = vec![header];

                if let Some(apc_state) = &device.apc_state {
                    lines.push(Line::from(vec![
                        Span::raw("water temp: "),
                        Span::styled(
                            format!("{:.1}", apc_state.temperature.water_temperature),
                            Style::default().fg(Color::Blue),
                        ),
                        Span::raw(" | heater temp: "),
                        Span::styled(
                            format!("{:.1}", apc_state.temperature.heater_temperature),
                            Style::default().fg(Color::Blue),
                        ),
                    ]));
                }

                Text::from(lines)
            }
            None => Text::from("No device selected"),
        };

        // add conditional color rendering based on if device or not.
        Paragraph::new(text)
            .block(Block::bordered().border_type(BorderType::Rounded))
            .fg(Color::DarkGray)
            .bg(Color::Black)
            .centered()
            .render(control_area, buf);

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
