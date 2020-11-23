use std::{
    //error::Error,
    io::{stdout, Write},
    sync::{mpsc, Arc, Mutex},
    thread,
    time::{Duration, Instant},
    path::Path,
};
use tui::Terminal;
use tui::widgets::TableState;
use tui::backend::{CrosstermBackend};
use argh::FromArgs;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use pcap::{Device, Capture};

mod draw;
mod model;
mod update;
use draw::draw;
use update::{update, Event};

/// Program options
#[derive(FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "250")]
    tick_rate: u64,

    /// read in a pcap file, or capture live
    #[argh(option, default = "String::from(\"\")", short = 'f')]
    filename: String,

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
        current_window: 0,
        should_quit: false,
        packets: Arc::new(Mutex::new(vec![])), 
        packets_to_draw: vec![],
        search_is_active: false,
        key_mode: model::KeyMode::Normal,
        packet_table_state: TableState::default(),
    };

    //initialize loop for sending program inputs
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
    let main_device = Device::lookup()?;
    if cli.filename.is_empty() {
        let mut cap = Capture::from_device(main_device).unwrap()
                          .promisc(true)
                          .timeout(20)
                          .open().unwrap();
        thread::spawn(move || {
            loop {
                while let Ok(packet) = cap.next() {
                    let newdata = packet.data.to_vec();
                    let newheader = model::PacketHeader {
                        ts: packet.header.ts,
                        caplen: packet.header.caplen,
                        len: packet.header.len,
                    };
                    let newpacket = Box::new(model::Packet {
                        header: newheader,
                        info: model::PacketInfo::from(newdata),
                    });
                    let _send_packet = packet_sender.send(Event::Traffic(newpacket));
                }
            }
        });
    } else {
        let capture_path = Path::new(&*cli.filename);
        let mut cap = Capture::from_file(capture_path).expect("Failed to find file");
        thread::spawn(move || {
            while let Ok(packet) = cap.next() {
                let newdata = packet.data.to_vec();
                let newheader = model::PacketHeader {
                    ts: packet.header.ts,
                    caplen: packet.header.caplen,
                    len: packet.header.len,
                };
                let newpacket = Box::new(model::Packet {
                    header: newheader,
                    info: model::PacketInfo::from(newdata),
                });
                let _send_packet = packet_sender.send(Event::Traffic(newpacket));
                thread::sleep(Duration::from_millis(10));
            };
        });
        
        
    }

    //The main application loop
    loop {
        let _update_the_model = update(&rx, &mut model);
        let _draw_the_window = terminal.draw(|f| draw(f, &mut model));
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
