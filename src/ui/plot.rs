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
use crate::utils::round_with_margin;

/// Pass temperature measurements here.
impl App {
    pub fn render_plot_page(&self, area: Rect, buf: &mut Buffer) {
        let device = match self.anova_devices.current_device() {
            None => {
                self.render_unavailable("no data (device not connected)", area, buf);
                return;
            }
            Some(device) => device,
        };

        let apc_state = match &device.apc_state {
            None => {
                self.render_unavailable("no data received yet", area, buf);
                return;
            }
            Some(apc_state) => apc_state,
        };

        //
        let vals: Vec<(f64, f64)> = device
            .temperature_values
            .v
            .iter()
            .enumerate()
            .map(|(i, v)| (i as f64, v.ceil()))
            .collect();

        // Only celsius for now.
        let target_temp = apc_state.state.job.target_temperature.0;
        let target_temp_data = [
            (vals.len().saturating_sub(1) as f64, target_temp),
            (vals.len() as f64, target_temp),
        ];

        let datasets = vec![
            // Use actual temperature data
            Dataset::default()
                .name("Water".italic())
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Blue))
                .graph_type(GraphType::Line)
                .data(&vals[..]),
            // Use actual target temperature.
            Dataset::default()
                .name("Target".italic())
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Green))
                .graph_type(GraphType::Line)
                .data(&target_temp_data[..]),
        ];

        let y_max = match vals.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()) {
            Some(max_val) => round_with_margin(max_val.1.max(target_temp), 1.2),
            None => target_temp,
        };

        let chart = Chart::new(datasets)
            .block(Block::bordered().title(Line::from("Temperature").cyan().bold().centered()))
            .x_axis(
                Axis::default()
                    .title("Progress")
                    .style(Style::default().gray())
                    .bounds([0.0, vals.len() as f64]),
            )
            .y_axis(
                Axis::default()
                    .title("Temperature")
                    .style(Style::default().gray())
                    .bounds([0.0, y_max])
                    .labels(["0".bold(), y_max.bold()]),
            )
            .legend_position(Some(LegendPosition::TopLeft))
            .hidden_legend_constraints((Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)));

        chart.render(area, buf);
    }
}
