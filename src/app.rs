use crate::{
    config::AppConfig,
    event::{AppEvent, Event, EventHandler},
};
use color_eyre::eyre;
use ignore::{WalkBuilder, types::TypesBuilder};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    widgets::ListState,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AppEntry {
    pub name: String,
    pub path: std::path::PathBuf,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,
    pub launchable_apps: Vec<AppEntry>,
    pub filtered_apps: Vec<AppEntry>,
    pub app_config: AppConfig,
    pub scan_duration: std::time::Duration,
    pub items_found: usize,
    pub list_state: ListState,
    pub filter_query: String,
}
impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> color_eyre::Result<Self> {
        let config = AppConfig::load().map_err(|e| eyre::eyre!(e))?;

        Ok(Self {
            running: true,
            events: EventHandler::new(),
            launchable_apps: vec![],
            filtered_apps: vec![],
            app_config: config,
            scan_duration: std::time::Duration::default(),
            items_found: usize::default(),
            list_state: ListState::default(),
            filter_query: String::default(),
        })
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.load_launchable_app();
        self.update_filter();

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
                AppEvent::ResetFilterQuery => self.reset_filter_query(),
                AppEvent::Execute => self.execute_selected(),
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
            KeyCode::Backspace => self.events.send(AppEvent::ResetFilterQuery),
            KeyCode::Esc => self.events.send(AppEvent::Quit),
            KeyCode::Down => self.events.send(AppEvent::Next),
            KeyCode::Up => self.events.send(AppEvent::Previous),
            KeyCode::Enter => self.events.send(AppEvent::Execute),
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
        self.update_filter();
    }

    fn reset_filter_query(&mut self) {
        if !self.filter_query.is_empty() {
            self.filter_query.clear();
            self.update_filter();
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

    fn execute_selected(&mut self) {
        if let Some(index) = self.list_state.selected() {
            if let Some(app_entry) = self.filtered_apps.get(index) {
                #[cfg(target_os = "windows")]
                let _ = std::process::Command::new("cmd")
                    .args(["/C", "start", "", &app_entry.path.to_string_lossy()])
                    .spawn();

                #[cfg(target_os = "macos")]
                let _ = std::process::Command::new("open")
                    .arg(&app_entry.path.to_string_lossy())
                    .spawn();

                #[cfg(target_os = "linux")]
                let _ = std::process::Command::new("xdg-open")
                    .arg(&app_entry.path.to_string_lossy())
                    .spawn();
            }
        }
    }

    fn update_filter(&mut self) {
        self.filtered_apps = self
            .launchable_apps
            .iter()
            .filter(|app| {
                app.name
                    .to_lowercase()
                    .contains(&self.filter_query.to_lowercase())
            })
            .cloned()
            .collect();

        //Select the firs titem if list not empty
        self.list_state.select(if self.filtered_apps.is_empty() {
            None
        } else {
            Some(0)
        });
    }

    fn load_launchable_app(&mut self) {
        let stopwatch = std::time::Instant::now();

        let mut types_builder = TypesBuilder::new();
        for ext in &self.app_config.watch_extensions {
            let _ = types_builder.add(ext, &format!("*.{}", ext));
        }

        let matcher = types_builder.select("all").build().unwrap();
        let (tx, rx) = std::sync::mpsc::channel();

        for path in &self.app_config.watch_paths {
            let tx = tx.clone();

            WalkBuilder::new(path)
                .hidden(self.app_config.skip_hidden_files)
                .types(matcher.clone())
                .build_parallel()
                .run(|| {
                    let tx = tx.clone();

                    Box::new(move |result| {
                        if let Ok(entry) = result {
                            if entry.file_type().map_or(false, |ft| ft.is_file()) {
                                let path = entry.path().to_path_buf();
                                let name = path
                                    .file_name()
                                    .map(|n| n.to_string_lossy().to_string())
                                    .unwrap_or_default();

                                let _ = tx.send(AppEntry { name, path });
                            }
                        }
                        ignore::WalkState::Continue
                    })
                });
        }

        drop(tx);

        let mut final_list: Vec<AppEntry> = rx.into_iter().collect();
        final_list = self.sort_by_ext_then_name(final_list);

        self.scan_duration = stopwatch.elapsed();
        self.launchable_apps = final_list;
        self.items_found = self.launchable_apps.len();
    }

    fn sort_by_ext_then_name(&mut self, mut final_list: Vec<AppEntry>) -> Vec<AppEntry> {
        final_list.sort_by(|a, b| {
            let ext_a = a.path.extension().and_then(|e| e.to_str()).unwrap_or("");
            let ext_b = b.path.extension().and_then(|e| e.to_str()).unwrap_or("");

            ext_a.cmp(ext_b).then_with(|| a.name.cmp(&b.name))
        });
        final_list
    }
}
