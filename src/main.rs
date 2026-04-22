use std::io;

use clap::Parser;
use color_eyre::Result;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

mod app;
mod codegen;
mod events;
mod registry;
mod ui;
mod wizard;

use app::App;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    pub dry_run: bool,

    #[arg(long, short)]
    pub output: Option<std::path::PathBuf>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, args);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, args: Args) -> Result<()> {
    let mut app = App::new(args);
    app.run(terminal)
        .map_err(|e| color_eyre::eyre::eyre!("{e:#}"))
}
