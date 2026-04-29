use crate::{
    anova_engine,
    event::{AppEvent, Event, EventHandler},
    types::{Devices, PageTab, PageTabs},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::DefaultTerminal;

#[derive(Debug)]
pub struct App {
    // app
    pub events: EventHandler,
    pub running: bool,
    // device
    pub anova_devices: Devices,
    // tabs
    pub page_tabs: PageTabs,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // app
            running: true,
            events: EventHandler::new(),
            // device
            anova_devices: Devices::mock(),
            // tabs
            page_tabs: PageTabs::new(),
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
        // send a clone of sender to device loop
        anova_engine::engine::start(self.events.sender.clone()).await?;
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
                    // only allow counter to change if in target tab.
                    AppEvent::Quit => self.quit(),
                    AppEvent::ChangeTab => self.page_tabs.next(),
                    AppEvent::NextDevice => self.anova_devices.next_device(),
                    AppEvent::PreviousDevice => self.anova_devices.previous_device(),
                    AppEvent::UpdateDevice => self.anova_devices.update_device(),
                    // set visible devices.
                    AppEvent::SetAppDevices(identified_devices) => {
                        self.anova_devices.update_devices(identified_devices);
                    }
                    // update apc state.
                    AppEvent::SetApcState(apc_state_simple) => {
                        self.anova_devices.set_apc_state(apc_state_simple);
                    }
                    _ => {}
                },
            }
        }
        Ok(())
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
    fn handle_control_events(&self, key_event: KeyEvent) {
        match key_event.code {
            _ => {}
        }
    }

    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        self.handle_global_events(key_event);

        match self.page_tabs.current_tab() {
            PageTab::Device => self.handle_device_events(key_event),
            PageTab::Control => self.handle_control_events(key_event),
        }

        Ok(())
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }
}
