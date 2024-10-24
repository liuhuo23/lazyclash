use clap::Parser;
use cli::Cli;
use color_eyre::Result;

use crate::app::App;

mod action;
mod app;
mod cli;
mod config;
mod errors;
mod logging;
mod menu;
mod mode;
mod prfitem;
mod utils;
mod view;

#[tokio::main]
async fn main() -> Result<()> {
    crate::errors::init()?;
    crate::logging::init()?;
    let _ = Cli::parse();
    let mut app = App::new()?;
    let res = if let Err(e) = app.run().await {
        eprint!("{} error: Something went wrong.", env!("CARGO_PKG_NAME"));
        Err(e)
    } else {
        Ok(())
    };
    app.exit()?;
    res
}
