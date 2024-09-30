mod app;
mod config;
mod functions;
use anyhow::Result;
use app::App;
fn main() -> Result<()> {
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}
