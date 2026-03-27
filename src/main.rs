use std::io::{self, IsTerminal, Read};
use std::path::PathBuf;

use clap::Parser;
use color_eyre::{eyre::eyre, Result};

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
    /// Markdown file to view (reads from stdin if omitted)
    file: Option<PathBuf>,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    let mut builder = app::App::builder();

    match cli.file {
        Some(path) => {
            builder = builder.file(path);
        }
        None => {
            if io::stdin().is_terminal() {
                return Err(eyre!("no file specified and nothing on stdin\n\nUsage: mdfi <file>\n       cat file.md | mdfi"));
            }

            let mut content = String::new();
            io::stdin().read_to_string(&mut content)?;

            #[cfg(unix)]
            reopen_stdin_from_tty()?;

            builder = builder.content(content, "(stdin)".into());
        }
    }

    let mut app = builder.build()?;
    app.run()
}

#[cfg(unix)]
fn reopen_stdin_from_tty() -> io::Result<()> {
    use std::os::unix::io::AsRawFd;

    let tty = std::fs::File::open("/dev/tty")?;
    let result = unsafe { libc::dup2(tty.as_raw_fd(), libc::STDIN_FILENO) };
    if result == -1 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}
