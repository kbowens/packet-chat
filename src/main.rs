use std::string::String;
use std::{
    //error::Error,
    io::{stdout, Write},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use tui::Terminal;
use tui::backend::{CrosstermBackend, Backend};
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};
use tui::Frame;
use argh::FromArgs;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use anyhow::{Result, Error};

struct Model {
	input: String,
	currentWindow: i8,
    should_quit: bool,
}

enum Event<I> {
    Input(I),
    Tick,
}

/// CLI input
#[derive(FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "250")]
    tick_rate: u64,

    /// whether unicode symbols are used to improve the overall look of the app
    #[argh(option, default = "true")]
    enhanced_graphics: bool,
}


fn main() -> anyhow::Result<()> {
	let cli: Cli = argh::from_env();

	enable_raw_mode()?;

    let (tx, rx) = mpsc::channel();

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut model = Model {
        input: "".to_string(), 
        currentWindow: 0,
        should_quit: false,
    };
    let mut terminal = Terminal::new(backend)?;
    loop {
        let _draw = terminal.draw(|f| draw(f, &mut model));
        update(&rx);
        if model.should_quit {
            break;
        }
    }

    Ok(())
}

fn draw<B: Backend>(f: &mut Frame<B>, model: &mut Model) {

}

fn update(rx: &mpsc::Receiver<Event<String>>) {

}