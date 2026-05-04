//! we can potentially put a .to_component() method
//! on ApcState to simplify some of the component generation.

use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::api::{ApcState, JobStatusState, TimeDisplay};
use crate::app::App;

use crate::types::{AnovaDevice, PageTab};
use ratatui::widgets::{List, ListItem, Tabs};

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
            _ => {}
        }
    }
}

impl App {
    /// Break this into better logic.
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

/// There are some bugs we need to fix:
/// * Switching C/F should convert the float as well.
/// * Fix progress bar when done cooking (and it starts counting up).
impl App {
    fn render_control_no_device_connected(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("No device connected")
            .block(Block::bordered().border_type(BorderType::Rounded))
            .fg(Color::DarkGray)
            .centered()
            .render(area, buf);
    }

    fn render_control_help(&self, area_h1: Rect, area_h2: Rect, buf: &mut Buffer) {
        Paragraph::new(Line::from(vec![
            "↹ ".magenta(),
            "change view".into(),
            " │ ".dark_gray(),
            "↵".cyan(),
            " start/stop".into(),
            " │ ".dark_gray(),
            "t".cyan(),
            " °C ↔ °F".into(),
        ]))
        .alignment(Alignment::Center)
        .render(area_h1, buf);

        Paragraph::new(Line::from(vec![
            "+/-".cyan(),
            " temp ↑↓".into(),
            " │ ".dark_gray(),
            "=/_".cyan(),
            " temp ↑↑↓↓".into(),
            " │ ".dark_gray(),
            "[/]".cyan(),
            " timer ↑↓".into(),
            " │ ".dark_gray(),
            "{/}".cyan(),
            " timer ↑↑↓↓".into(),
        ]))
        .alignment(Alignment::Center)
        .render(area_h2, buf);
    }

    fn render_control_temperature(&self, apc_state: &ApcState, temp_area: Rect, buf: &mut Buffer) {
        let temp_lines = vec![
            Line::from(vec![
                Span::styled("water   ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    apc_state
                        .temperature_info
                        .water_temperature
                        .to_display(&apc_state.job.temperature_unit),
                    Style::default().fg(Color::Blue).bold(),
                ),
            ]),
            Line::from(vec![
                Span::styled("heater  ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    apc_state
                        .temperature_info
                        .heater_temperature
                        .to_display(&apc_state.job.temperature_unit),
                    Style::default().fg(Color::Red),
                ),
            ]),
            Line::from(vec![
                Span::styled("triac   ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    apc_state
                        .temperature_info
                        .triac_temperature
                        .to_display(&apc_state.job.temperature_unit),
                    Style::default().fg(Color::Gray),
                ),
            ]),
        ];

        Paragraph::new(temp_lines)
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title(Span::styled(
                        " temperature ",
                        Style::default().fg(Color::Green),
                    ))
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
            .render(temp_area, buf);
    }

    fn render_control_header(&self, device: &AnovaDevice, header_area: Rect, buf: &mut Buffer) {
        let divider = Span::styled(" │ ", Style::default().fg(Color::DarkGray));

        let connection_span = if device.is_connected() {
            Span::styled("● Connected", Style::default().fg(Color::Green))
        } else {
            Span::styled("○ Disconnected", Style::default().fg(Color::Red))
        };

        let running_span = if device.is_running() {
            Span::styled("● Cooking", Style::default().fg(Color::Green)).rapid_blink()
        } else {
            Span::styled("○ Idle", Style::default().fg(Color::DarkGray))
        };

        let header_line = Line::from(vec![
            Span::styled(
                format!(" id: {}", device.cooker_id),
                Style::default().fg(Color::DarkGray),
            ),
            divider.clone(),
            Span::styled(&device.name, Style::default().fg(Color::Gray)),
            divider.clone(),
            connection_span,
            divider.clone(),
            running_span,
            Span::raw(" "),
        ]);

        Paragraph::new(header_line)
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
            .render(header_area, buf);
    }

    fn render_control_job(&self, apc_state: &ApcState, area: Rect, buf: &mut Buffer) {
        let job = &apc_state.job;
        let job_status = &apc_state.job_status;

        // not sure I like this logic, but seems to work for now.
        let remaining_secs = match &job_status.state {
            JobStatusState::Maintaining => 0,
            _ => job_status.cook_time_remaining,
        };

        let total_secs = job.cook_time_seconds;
        let elapsed = total_secs.saturating_sub(remaining_secs);
        let pct = if total_secs > 0 {
            (elapsed as f64 / total_secs as f64).clamp(0.0, 1.0)
        } else {
            0.0
        };

        let time_display = match job_status.state {
            JobStatusState::Cooking | JobStatusState::PreHeating | JobStatusState::Empty => {
                vec![
                    Span::styled("remain  ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        job_status.cook_time_remaining.to_display(),
                        Style::default().fg(Color::Yellow),
                    ),
                    Span::styled(
                        format!(" / {}", total_secs.to_display()),
                        Style::default().fg(Color::DarkGray),
                    ),
                ]
            }
            JobStatusState::Maintaining => {
                vec![
                    Span::styled("overtime  ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        job_status.cook_time_remaining.to_display(),
                        Style::default().fg(Color::Red),
                    ),
                ]
            }
        };

        let mut job_lines = vec![
            Line::from(vec![
                Span::styled("state   ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    job_status.state.to_string(),
                    Style::default().fg(Color::Green),
                ),
            ]),
            Line::from(vec![
                Span::styled("mode    ", Style::default().fg(Color::DarkGray)),
                Span::styled(job.mode.to_string(), Style::default().fg(Color::Gray)),
            ]),
            Line::from(vec![
                Span::styled("target  ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    job.target_temperature.to_display(&job.temperature_unit),
                    Style::default().fg(Color::Yellow),
                ),
            ]),
            Line::from(time_display),
            Line::from(vec![
                Span::styled("job id  ", Style::default().fg(Color::DarkGray)),
                Span::styled(&job.id, Style::default().fg(Color::DarkGray)),
            ]),
        ];

        if matches!(
            job_status.state,
            JobStatusState::PreHeating | JobStatusState::Cooking
        ) {
            let bar_width = (area.width as usize).saturating_sub(1);
            let filled = (pct * bar_width as f64) as usize;
            let bar = format!(
                "{}{}",
                "█".repeat(filled),
                "░".repeat(bar_width.saturating_sub(filled))
            );

            let progress_bar =
                Line::from(vec![Span::styled(bar, Style::default().fg(Color::Green))]);

            job_lines.push(Line::from(Span::raw(" ")));
            job_lines.push(Line::from(Span::styled("Progress", Style::default())).centered());
            job_lines.push(progress_bar);
        }

        Paragraph::new(job_lines)
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title(Span::styled(" job ", Style::default().fg(Color::Magenta)))
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
            .render(area, buf);
    }

    fn render_network_page(&self, apc_state: &ApcState, area: Rect, buf: &mut Buffer) {
        let net = &apc_state.network_info;

        let net_lines = vec![
            Line::from(vec![
                Span::styled("ssid    ", Style::default().fg(Color::DarkGray)),
                Span::styled(&net.ssid, Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::styled("status  ", Style::default().fg(Color::DarkGray)),
                Span::styled(&net.connection_status, Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled("mac     ", Style::default().fg(Color::DarkGray)),
                Span::styled(&net.mac_address, Style::default().fg(Color::Gray)),
            ]),
            Line::from(vec![
                Span::styled("mode    ", Style::default().fg(Color::DarkGray)),
                Span::styled(&net.mode, Style::default().fg(Color::Gray)),
            ]),
        ];

        Paragraph::new(net_lines)
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title(Span::styled(" network ", Style::default().fg(Color::Cyan)))
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
            .render(area, buf);
    }

    fn render_pin_page(&self, apc_state: &ApcState, area: Rect, buf: &mut Buffer) {
        let pins = &apc_state.pin_info;

        let ok = Style::default().fg(Color::Green);
        let err = Style::default().fg(Color::Red);

        let pin_flag = |v: u32, label: &'static str| -> Line {
            let (sym, sty) = if v == 0 { ("✓", ok) } else { ("✗", err) };
            Line::from(vec![
                Span::styled(
                    format!("{:<14}", label),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(sym, sty),
            ])
        };

        let pin_lines = vec![
            pin_flag(1u32.saturating_sub(pins.device_safe), "device safe"),
            pin_flag(pins.water_leak, "water leak"),
            pin_flag(pins.water_level_low, "water low"),
            pin_flag(pins.water_level_critical, "water crit"),
            pin_flag(pins.motor_stuck, "motor stuck"),
        ];

        Paragraph::new(pin_lines)
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title(Span::styled(
                        " pin info ",
                        Style::default().fg(Color::Yellow),
                    ))
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
            .render(area, buf);
    }

    fn render_sys_info(&self, apc_state: &ApcState, area: Rect, buf: &mut Buffer) {
        let sys = &apc_state.system_info_2640;

        let divider = Span::styled(" │ ", Style::default().fg(Color::DarkGray));

        let sha = &sys.firmware_version_sha;
        let sha_short = &sha[..7.min(sha.len())];
        let heap_free = sys.total_free_heap_size / 1024;
        let heap_total = sys.total_heap_size / 1024;

        let sysinfo_line = Line::from(vec![
            Span::styled(" fw ", Style::default().fg(Color::DarkGray)),
            Span::styled(&sys.firmware_version, Style::default().fg(Color::Gray)),
            Span::styled(
                format!(" ({sha_short})"),
                Style::default().fg(Color::DarkGray),
            ),
            divider.clone(),
            Span::styled("heap ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{heap_free} KB / {heap_total} KB"),
                Style::default().fg(Color::Gray),
            ),
            divider.clone(),
            Span::styled("mcu ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                sys.mcu_temperature
                    .to_display(&apc_state.job.temperature_unit),
                Style::default().fg(Color::Gray),
            ),
        ]);

        Paragraph::new(sysinfo_line)
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
            .render(area, buf);
    }

    fn render_set_values(&self, apc_state: &ApcState, area: Rect, buf: &mut Buffer) {
        let ctrl_lines = vec![
            Line::from(vec![
                Span::styled("set temp  ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    self.device_control
                        .set_temperature
                        .to_display(&apc_state.job.temperature_unit),
                    Style::default().fg(Color::Yellow).bold(),
                ),
            ]),
            Line::from(vec![
                Span::styled("set timer ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    self.device_control.set_timer.to_display(),
                    Style::default().fg(Color::Yellow).bold(),
                ),
            ]),
        ];

        Paragraph::new(ctrl_lines)
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title(Span::styled(
                        " controls ",
                        Style::default().fg(Color::Yellow),
                    ))
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
            .render(area, buf);
    }

    /// Main renderer.
    fn render_control_page(&self, area: Rect, buf: &mut Buffer) {
        // main layout
        let [
            header_area,
            body_area,
            sysinfo_area,
            help_area_1,
            help_area_2,
        ] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .areas(area);

        // early break if device does not exist.
        let device = match self.anova_devices.current_device() {
            None => {
                self.render_control_no_device_connected(header_area, buf);
                return;
            }
            Some(device) => device,
        };

        // basic device state
        self.render_control_header(device, header_area, buf);

        // early break if apc state does not exist
        let apc = match &device.apc_state {
            Some(apc_state) => apc_state,
            None => return,
        };

        // body layout
        let [left_col, right_area] =
            Layout::horizontal([Constraint::Fill(2), Constraint::Fill(4)]).areas(body_area);

        let [temp_panel, set_panel] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(7)]).areas(left_col);

        let [job_panel, right_panels] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(right_area);

        let [network_panel, pin_panel] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(right_panels);

        // render body.
        self.render_control_temperature(&apc.state, temp_panel, buf);
        self.render_control_job(&apc.state, job_panel, buf);
        self.render_network_page(&apc.state, network_panel, buf);
        self.render_pin_page(&apc.state, pin_panel, buf);
        self.render_sys_info(&apc.state, sysinfo_area, buf);
        self.render_set_values(&apc.state, set_panel, buf);
        self.render_control_help(help_area_1, help_area_2, buf);
    }
}
