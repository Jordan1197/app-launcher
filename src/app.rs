use std::sync::{Arc, Mutex};

use crate::{
    config::AppConfig,
    event::{AppEvent, Event, EventHandler},
};
use ignore::WalkBuilder;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    widgets::ListState,
};

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,
    pub launchable_apps: Vec<String>, //TODO : Va devoir être un object (name pour affichache, fullpath pour launcher)
    pub app_config: AppConfig,
    pub scan_duration: std::time::Duration,
    pub items_found: usize,
    pub list_state: ListState,
    pub filter_query: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            events: EventHandler::new(),
            launchable_apps: vec![],
            app_config: AppConfig::load().expect("Failed to load config"),
            scan_duration: std::time::Duration::default(),
            items_found: usize::default(),
            list_state: ListState::default(),
            filter_query: String::default(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.load_launchable_app();

        while self.running {
            terminal.draw(|frame| {
                crate::ui::render(&mut self, frame);
            })?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next()? {
            Event::Tick => self.tick(),
            Event::Crossterm(event) => match event {
                crossterm::event::Event::Key(key_event)
                    if key_event.kind == crossterm::event::KeyEventKind::Press =>
                {
                    self.handle_key_event(key_event)?
                }
                _ => {}
            },
            Event::App(app_event) => match app_event {
                AppEvent::Next => self.next(),
                AppEvent::Previous => self.previous(),
                AppEvent::AddFilterQuery(c) => self.add_filter_query(c),
                AppEvent::RemoveFilterQuery => self.remove_filter_query(),
                AppEvent::ResetFilterQuery => self.reset_filter_query(),
                AppEvent::Quit => self.quit(),
            },
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Char(c) => self.events.send(AppEvent::AddFilterQuery(c)),
            KeyCode::Backspace => self.events.send(AppEvent::RemoveFilterQuery),
            KeyCode::Esc => self.events.send(AppEvent::ResetFilterQuery),
            KeyCode::Down => self.events.send(AppEvent::Next),
            KeyCode::Up => self.events.send(AppEvent::Previous),
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    fn tick(&self) {}

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }

    fn add_filter_query(&mut self, c: char) {
        self.filter_query.push(c);
    }

    fn remove_filter_query(&mut self) {
        self.filter_query.pop();
    }

    fn reset_filter_query(&mut self) {
        if !self.filter_query.is_empty() {
            self.filter_query.clear();
        } else {
            self.quit();
        }
    }

    fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.launchable_apps.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.launchable_apps.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn load_launchable_app(&mut self) {
        let stopwatch = std::time::Instant::now();
        let apps = Arc::new(Mutex::new(Vec::new()));

        for path in &self.app_config.watch_paths {
            let apps_clone = Arc::clone(&apps);

            WalkBuilder::new(path)
                .hidden(self.app_config.skip_hidden_files)
                .build_parallel()
                .run(|| {
                    let apps_thread = Arc::clone(&apps_clone);
                    Box::new(move |result| {
                        if let Ok(entry) = result {
                            if entry.path().extension().map_or(false, |ext| ext == "sln") {
                                //TODO: config pour l'extension, maybe plusieurs ?
                                let path_str = entry.path().to_string_lossy().to_string();
                                apps_thread.lock().unwrap().push(path_str);
                            }
                        }
                        ignore::WalkState::Continue
                    })
                });
        }

        let mut final_list = Arc::try_unwrap(apps).unwrap().into_inner().unwrap();
        final_list.sort();

        self.scan_duration = stopwatch.elapsed();
        self.launchable_apps = final_list;
        self.items_found = self.launchable_apps.len();

        if !self.launchable_apps.is_empty() {
            self.list_state.select(Some(0));
        }
    }
}
