use crate::{
    anova_engine,
    api::{ApcSetPayload, ApcStartPayload, ApcStopPayload, ApiRequest, TemperatureUnit},
    event::{AppEvent, Event, EventHandler},
    types::{DeviceControl, Devices, FixedValueSet, PageTab},
};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::DefaultTerminal;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{debug, error, instrument, warn};

#[derive(Debug)]
pub struct App {
    // app
    pub events: EventHandler,
    pub running: bool,
    pub api_sender: Option<UnboundedSender<ApiRequest>>,
    // device
    pub anova_devices: Devices,
    // device control
    pub device_control: DeviceControl,
    // tabs
    pub page_tabs: FixedValueSet<PageTab>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // app
            running: true,
            events: EventHandler::new(),
            api_sender: None,
            // device
            anova_devices: Devices::new(),
            // device control
            device_control: DeviceControl::default(),
            // tabs
            page_tabs: FixedValueSet::<PageTab>::new(Some(0), Some(0)),
        }
    }
}

/// We need to:
/// * Implement the wss api request for start/set/end.
/// * Make the control UI nice, should be able to choose temp, etc.
impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        // We can probably move this somewhere else.
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<ApiRequest>();
        self.api_sender = Some(tx);

        // We can probably move this somewhere else
        anova_engine::engine::start(self.events.sender.clone(), rx).await?;

        //
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;

            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event)
                        if key_event.kind == crossterm::event::KeyEventKind::Press =>
                    {
                        self.handle_key_events(key_event)?
                    }
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    // global
                    AppEvent::Quit => self.quit(),
                    AppEvent::ChangeTab => self.page_tabs.increment_set(),
                    // device
                    AppEvent::NextDevice => self.anova_devices.next_device(),
                    AppEvent::PreviousDevice => self.anova_devices.previous_device(),
                    AppEvent::UpdateDevice => self.anova_devices.update_device(),
                    AppEvent::SetAppDevices(identified_devices) => {
                        self.anova_devices.update_devices(identified_devices);
                    }
                    AppEvent::SetApcState(apc_state_simple) => {
                        self.anova_devices.set_apc_state(apc_state_simple);
                    }
                    // api requests
                    AppEvent::StartOrStop => self.send_start_or_stop_request(),
                    AppEvent::SwitchTemperatureUnit => self.send_set_request(),
                    // device control
                    AppEvent::IncrementSetTemperature(delta) => {
                        self.device_control.increment_temperature(delta)
                    }
                    AppEvent::DecrementSetTemperature(delta) => {
                        self.device_control.decrement_temperature(delta)
                    }
                    AppEvent::IncrementSetTimer(delta) => {
                        self.device_control.increment_timer(delta)
                    }
                    AppEvent::DecrementSetTimer(delta) => {
                        self.device_control.decrement_timer(delta)
                    }

                    _ => {}
                },
            }
        }
        Ok(())
    }

    #[instrument]
    pub fn send_start_or_stop_request(&mut self) {
        let (api_sender, device) = match (&self.api_sender, self.anova_devices.current_device()) {
            (Some(api_sender), Some(device)) => (api_sender, device),
            _ => {
                warn!(
                    "cannot send API request - api_sender: {:?}, current_device: {:?}",
                    self.api_sender,
                    self.anova_devices.current_device()
                );
                return;
            }
        };

        let api_request = match device.is_running() {
            false => ApiRequest::Start(ApcStartPayload {
                cooker_id: device.cooker_id.clone(),
                r#type: device.r#type.clone(),
                target_temperature: self.device_control.set_temperature,
                unit: TemperatureUnit::C,
                timer: self.device_control.set_timer,
            }),
            true => ApiRequest::Stop(ApcStopPayload {
                cooker_id: device.cooker_id.clone(),
                r#type: device.r#type.clone(),
            }),
        };

        debug!("sending {:?}", api_request);
        match api_sender.send(api_request) {
            Ok(_) => debug!("succeeded"),
            Err(e) => error!("failed to send `{e}`"),
        }
    }

    #[instrument]
    pub fn send_set_request(&mut self) {
        let (api_sender, device) = match (&self.api_sender, self.anova_devices.current_device()) {
            (Some(api_sender), Some(device)) => (api_sender, device),
            _ => {
                warn!(
                    "cannot send API request - api_sender: {:?}, current_device: {:?}",
                    self.api_sender,
                    self.anova_devices.current_device()
                );
                return;
            }
        };

        let new_temperature_unit = match device.current_temperature_unit() {
            Some(TemperatureUnit::C) => TemperatureUnit::F,
            Some(TemperatureUnit::F) => TemperatureUnit::C,
            _ => {
                warn!("No temperature unit available");
                return;
            }
        };

        let api_request = ApiRequest::Set(ApcSetPayload {
            cooker_id: device.cooker_id.clone(),
            r#type: device.r#type.clone(),
            unit: new_temperature_unit,
        });

        debug!("sending {:?}", api_request);

        match api_sender.send(api_request) {
            Ok(_) => debug!("succeeded"),
            Err(e) => error!("failed to send: `{e}`"),
        }
    }

    fn handle_global_events(&mut self, key_event: KeyEvent) {
        match key_event.code {
            // quit through esc, q or ctrl + c.
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('c' | 'C')
                if key_event.modifiers == KeyModifiers::CONTROL =>
            {
                self.events.send(AppEvent::Quit)
            }
            // switch tab.
            KeyCode::Tab => self.events.send(AppEvent::ChangeTab),

            _ => {}
        }
    }

    fn handle_device_events(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Enter => self.anova_devices.update_device(),
            KeyCode::Down => self.anova_devices.next_device(),
            KeyCode::Up => self.anova_devices.previous_device(),
            _ => {}
        }
    }

    fn handle_control_events(&mut self, key_event: KeyEvent) {
        match key_event.code {
            // for now, mock to make sure it works
            KeyCode::Char('S') | KeyCode::Char('s') => self.events.send(AppEvent::StartOrStop),
            KeyCode::Char('T') | KeyCode::Char('t') => {
                self.events.send(AppEvent::SwitchTemperatureUnit)
            }
            KeyCode::Char('+') => self.events.send(AppEvent::IncrementSetTemperature(0.5)),
            KeyCode::Char('-') => self.events.send(AppEvent::DecrementSetTemperature(0.5)),
            KeyCode::Char('=') => self.events.send(AppEvent::IncrementSetTemperature(2.0)),
            KeyCode::Char('_') => self.events.send(AppEvent::DecrementSetTemperature(2.0)),
            KeyCode::Char(']') => self.events.send(AppEvent::IncrementSetTimer(1)),
            KeyCode::Char('[') => self.events.send(AppEvent::DecrementSetTimer(1)),
            KeyCode::Char('}') => self.events.send(AppEvent::IncrementSetTimer(60)),
            KeyCode::Char('{') => self.events.send(AppEvent::DecrementSetTimer(60)),

            _ => {}
        }
    }

    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        self.handle_global_events(key_event);

        match self.page_tabs.current() {
            Some(PageTab::Device) => self.handle_device_events(key_event),
            Some(PageTab::Control) => self.handle_control_events(key_event),
            _ => {}
        }

        Ok(())
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }
}
