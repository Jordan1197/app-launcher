mod app;
mod config;
mod event;
mod helpers;
mod ui;

use crate::app::App;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    run_app()
}

fn run_app() -> color_eyre::Result<()> {
    let app_result = App::new();

    match app_result {
        Ok(app) => {
            let terminal = ratatui::init();
            let result = app.run(terminal);
            ratatui::restore();
            result
        }
        Err(e) => {
            eprintln!("--------------------------------------------------");
            eprintln!("APPLICATION STARTUP ERROR");
            eprintln!("--------------------------------------------------");
            eprintln!("\n{}", e);
            eprintln!("\nPress Enter to close this window...");

            let mut pause = String::new();
            std::io::stdin().read_line(&mut pause)?;

            // Exit with the error code
            Err(e)
        }
    }
}
