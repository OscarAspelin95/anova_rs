use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::style::Style;
use ratatui::symbols;
use ratatui::text::{Line, Span};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    widgets::{Block, Widget},
};

use ratatui::widgets::{
    Axis, Borders, Chart, Dataset, GraphType, LegendPosition, Padding, Paragraph,
};

use crate::api::apc::events::apc_state::ApcStatePayload;
use crate::api::{Celsius, TimeDisplay};
use crate::app::App;
use crate::utils::{RestrictedVecDeque, round_with_margin};

impl App {
    pub fn render_chart_help(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(Line::from(vec![
            "↹ ".magenta(),
            "change view".into(),
            " │ ".dark_gray(),
            "t".cyan(),
            " °C ↔ °F".into(),
        ]))
        .alignment(Alignment::Center)
        .render(area, buf);
    }

    pub fn render_charts(
        &self,
        temperature_values: &RestrictedVecDeque<Celsius, 100>,
        apc_state: &ApcStatePayload,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let vals: Vec<(f64, f64)> = temperature_values
            .v
            .iter()
            .enumerate()
            .map(|(i, v)| (i as f64, v.to_f64(&apc_state.state.job.temperature_unit)))
            .collect();

        // need to make this better to avoid nexted apc_state requirement.
        // put a method directly on apc_stats for target_temperature_f64.
        let target_temp = apc_state
            .state
            .job
            .target_temperature
            .to_f64(&apc_state.state.job.temperature_unit)
            .ceil();

        let target_temp_data = [
            (vals.len().saturating_sub(1) as f64, target_temp),
            (vals.len() as f64, target_temp),
        ];

        let datasets = vec![
            Dataset::default()
                .name("Target".italic())
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Yellow))
                .graph_type(GraphType::Line)
                .data(&target_temp_data[..]),
            Dataset::default()
                .name("Water".italic())
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Blue))
                .graph_type(GraphType::Line)
                .data(&vals[..]),
        ];

        let y_max = match vals.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()) {
            Some(max_val) => round_with_margin(max_val.1.max(target_temp), 1.2),
            None => target_temp,
        };

        let y_min = match vals.iter().min_by(|a, b| a.1.partial_cmp(&b.1).unwrap()) {
            Some(min_val) => round_with_margin(min_val.1.min(target_temp), 0.80),
            None => target_temp,
        };

        let timer_display = apc_state.state.job_status.cook_time_remaining.to_display();

        let title = Line::from(vec![
            Span::styled("Water Temp:	", Style::default().dark_gray()),
            Span::styled(
                apc_state
                    .state
                    .temperature_info
                    .heater_temperature
                    .to_display(&apc_state.state.job.temperature_unit),
                Style::default().blue(),
            ),
            Span::raw(" | "),
            Span::styled("Target:	", Style::default().dark_gray()),
            Span::styled(
                apc_state
                    .state
                    .job
                    .target_temperature
                    .to_display(&apc_state.state.job.temperature_unit),
                Style::default().fg(Color::Yellow),
            ),
            Span::raw(" | "),
            Span::styled("Timer: ", Style::default().dark_gray()),
            Span::styled(timer_display, Style::default().fg(Color::DarkGray)),
        ]);

        let chart = Chart::new(datasets)
            .block(
                Block::new()
                    .borders(Borders::empty())
                    .title(title)
                    .padding(Padding::horizontal(2))
                    .style(Style::default().dark_gray())
                    .title_alignment(Alignment::Center),
            )
            .x_axis(
                Axis::default()
                    .title("Progress")
                    .style(Style::default().dark_gray())
                    .bounds([0.0, vals.len() as f64])
                    .labels([""]),
            )
            .y_axis(
                Axis::default()
                    .title(format!(
                        "Temp ({})",
                        apc_state.state.job.temperature_unit.to_display()
                    ))
                    .style(Style::default().dark_gray())
                    .bounds([y_min, y_max])
                    .labels([y_min.bold(), y_max.bold()]),
            )
            .legend_position(Some(LegendPosition::TopLeft))
            .hidden_legend_constraints((Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)));

        chart.render(area, buf);
    }

    pub fn render_chart_page(&self, area: Rect, buf: &mut Buffer) {
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

        let [plot_area, help_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);

        self.render_chart_help(help_area, buf);
        self.render_charts(&device.temperature_values, apc_state, plot_area, buf);
    }
}
