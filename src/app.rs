use crate::{
    anova_engine,
    event::{AppEvent, Event, EventHandler},
    types::{
        ApiRequest, ControlType, Devices, FixedValueSet, PageTab,
        api_request::{self, ApcSetPayload, ApcStartPayload, ApcStopPayload},
    },
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::DefaultTerminal;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug)]
pub struct App {
    // app
    pub events: EventHandler,
    pub running: bool,
    pub api_sender: Option<UnboundedSender<ApiRequest>>,
    // device
    pub anova_devices: Devices,
    // control
    pub control: FixedValueSet<ControlType>,
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
            // control
            control: FixedValueSet::<ControlType>::new_empty(),
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
                    // control
                    AppEvent::NextControl => self.control.increment(),
                    AppEvent::PreviousControl => self.control.decrement(),
                    AppEvent::UpdateControl => self.control.set(),
                    AppEvent::SendApiRequest => self.send_api_request(),
                    _ => {}
                },
            }
        }
        Ok(())
    }

    /// Catch or log potential errors later on.
    /// TEMP - test that it works.
    pub fn send_api_request(&mut self) {
        match (
            &self.api_sender,
            self.anova_devices.current_device(),
            self.control.current(),
        ) {
            (Some(api_sender), Some(device), Some(control)) => {
                let api_request = match control {
                    ControlType::Start => ApiRequest::Start(ApcStartPayload {
                        cooker_id: device.cooker_id.clone(),
                        r#type: device.r#type.clone(),
                        target_temperature: 35.0,
                        unit: api_request::TemperatureUnit::C,
                        timer: 100,
                    }),

                    ControlType::Set => ApiRequest::Set(ApcSetPayload {
                        cooker_id: device.cooker_id.clone(),
                        r#type: device.r#type.clone(),
                        unit: api_request::TemperatureUnit::C,
                    }),
                    ControlType::Stop => ApiRequest::Stop(ApcStopPayload {
                        cooker_id: device.cooker_id.clone(),
                        r#type: device.r#type.clone(),
                    }),
                };

                let _ = api_sender.send(api_request);
            }
            _ => {}
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
            KeyCode::Enter => self.events.send(AppEvent::SendApiRequest),
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
