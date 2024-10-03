mod app;
mod config;
mod functions;
mod menus;
use anyhow::Result;
use app::App;
fn main() -> Result<()> {
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}
