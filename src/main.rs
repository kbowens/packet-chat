use std::string::String;
use std::{
    //error::Error,
    io::{stdout, Write},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use tui::Terminal;
use tui::backend::{CrosstermBackend};
use argh::FromArgs;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use pcap::{Device, Capture, Packet};

mod draw;
mod model;
use draw::draw;
use model::Model;

enum Event<I, P> {
    Input(I),
    Tick,
    Packet(P),
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

    //Set up cli environment
	let cli: Cli = argh::from_env();
	enable_raw_mode()?;
    //message producer and consumer
    let (tx, rx) = mpsc::channel();
    let packet_sender = tx.clone();

    //create context for new crossterm window
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    //initalize model from the Cli struct
    let mut model = model::Model {
        input: "".to_string(), 
        currentWindow: 0,
        should_quit: false,
        packets: vec![], 
    };

    //initialize getevent loop
    let tick_rate = Duration::from_millis(cli.tick_rate);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if event::poll(timeout).unwrap() {
                let new_event: Result<CEvent, crossterm::ErrorKind> = event::read();
                match new_event {
                    Ok(event) => {let _ = tx.send(Event::Input(event)); }
                    _ => { }
                }
            }
            if last_tick.elapsed() >= tick_rate {
                tx.send(Event::Tick).unwrap();
                last_tick = Instant::now();
            }
        }
    });

    //start capturing traffic
    let main_device = Device::lookup().unwrap();
    
    thread::spawn(move || {
        let mut cap = Capture::from_device(main_device).unwrap()
                      .promisc(true)
                      .timeout(20)
                      .open().unwrap();
        while let Ok(packet) = cap.next() {
            packet_sender.send(Event::Packet(Box::new(packet)));
        }
    });

    //The main application loop
    loop {
        let _draw = terminal.draw(|f| draw(f, &model));
        update(&rx, &mut model);
        if model.should_quit {
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
                )?;
            break;
        }
    }

    Ok(())
}



fn update(rx: &mpsc::Receiver<Event<CEvent, Box<Packet>>>, model: &mut Model) -> anyhow::Result<()> {
    match rx.recv()? {
        Event::Input(event) => match event {
            CEvent::Key(kevent) => {
                model.should_quit = true;
            }, 
            CEvent::Mouse(mevent) => {

            },
            CEvent::Resize(rh, rw) => {

            }
        },
        Event::Tick => {

        }, 
        Event::Packet(packet) => {

        }
    }

    Ok(())
}