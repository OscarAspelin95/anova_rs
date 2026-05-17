use ratatui::layout::Constraint;
use ratatui::style::Style;
use ratatui::symbols;
use ratatui::text::Line;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    widgets::{Block, Widget},
};

use ratatui::widgets::{Axis, Chart, Dataset, GraphType, LegendPosition};

use crate::app::App;

/// Pass temperature measurements here.
impl App {
    pub fn render_plot_page(&self, area: Rect, buf: &mut Buffer) {
        let datasets = vec![
            // Use actual temperature data
            Dataset::default()
                .name("Water".italic())
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Blue))
                .graph_type(GraphType::Line)
                .data(&[(1., 1.), (4., 4.)]),
            // Use actual target temperature.
            Dataset::default()
                .name("Target".italic())
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Green))
                .graph_type(GraphType::Line)
                .data(&[(1., 4.), (4., 4.)]),
        ];

        let chart = Chart::new(datasets)
            .block(Block::bordered().title(Line::from("Temperature").cyan().bold().centered()))
            .x_axis(
                Axis::default()
                    .title("Time")
                    .style(Style::default().gray())
                    .bounds([0.0, 5.0])
                    .labels(["0".bold(), "2.5".into(), "5.0".bold()]),
            )
            .y_axis(
                Axis::default()
                    .title("Temperature")
                    .style(Style::default().gray())
                    // Must catch edge case where water temp has been larger
                    // than current target temp (e.g., if owering target temp).
                    .bounds([0.0, 5.0])
                    .labels(["0".bold(), "2.5".into(), "5.0".bold()]),
            )
            .legend_position(Some(LegendPosition::TopLeft))
            .hidden_legend_constraints((Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)));

        chart.render(area, buf);
    }
}
