use std::path::PathBuf;

use clap::Parser;
use color_eyre::Result;

mod action;
mod app;
mod event;
mod input;
mod markdown;
mod terminal;
mod ui;

#[derive(Parser)]
#[command(name = "mdfi", about = "A TUI markdown viewer", version)]
struct Cli {
    file: PathBuf,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    let mut app = app::App::builder().file(cli.file).build()?;

    app.run()
}
