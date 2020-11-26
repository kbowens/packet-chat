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
use stfu8;
use chrono::naive::NaiveDateTime;
use pallet;


mod draw;
mod model;
mod update;
mod search;
mod packetthread;
mod dbthread;
mod util;
use util::{process_packets};
use packetthread::handle_packets;
use dbthread::handle_db;
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
    let (update_tx, update_rx) = mpsc::channel();
    let (packet_sender, packet_receiver) = mpsc::channel();

    //create context for new crossterm window
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    //initalize model from the Cli struct
    let mut model = Arc::new(Mutex::new(model::Model {
        input: "".to_string(), 
        current_window: 0,
        should_quit: false,
        packets: Arc::new(Mutex::new(vec![])), 
        packets_to_draw: Arc::new(Mutex::new(vec![])),
        search_is_active: false,
        key_mode: model::KeyMode::Normal,
        packet_table_state: TableState::default(),
        gauge_ratio: Arc::new(Mutex::new(0)),
        new_packets: Arc::new(Mutex::new(vec![])),
        selected_packet: None,
    }));

    //Create Pallet Database
    let temp_dir = tempfile::TempDir::new_in(".")?;
    let db = pallet::ext::sled::open(temp_dir.path().join("db"))?;
    let packetstore: pallet::Store<model::Packet> = pallet::Store::builder()
        .with_db(db)
        .with_index_dir(temp_dir.path())
        .finish()?;

    //initialize loop for sending program inputs
    let tick_rate = Duration::from_millis(cli.tick_rate);
    let tupdate_tx = update_tx.clone();
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if event::poll(timeout).unwrap() {
                let new_event: Result<CEvent, crossterm::ErrorKind> = event::read();
                match new_event {
                    Ok(event) => {let _ = tupdate_tx.send(Event::Input(event)); }
                    _ => { }
                }
            }
            if last_tick.elapsed() >= tick_rate {
                tupdate_tx.send(Event::Tick).unwrap();
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
                process_packets(&mut cap, &packet_sender);
            }
        });
    } else {
        let capture_path = Path::new(&*cli.filename);
        let mut cap = Capture::from_file(capture_path).expect("Failed to find file");
        thread::spawn(move || {
                process_packets(&mut cap, &packet_sender);
                thread::sleep(Duration::from_millis(10));
        });
        
    }

    //create the thread to handle packets and db
    let pthreadmodel = model.clone();
    let pthreadupdate = update_tx.clone();
    let pmodel = model.clone();
    let packets_list_for_threads: Arc<Mutex<Vec<model::Packet>>> = Arc::new(Mutex::new(vec![]));
    let ptpacketlist = packets_list_for_threads.clone();
    let dbpacketlist = packets_list_for_threads.clone();
    thread::spawn(move || handle_packets(&packet_receiver, &pthreadupdate, pthreadmodel, ptpacketlist));
    thread::spawn(move || handle_db(pmodel, packets_list_for_threads));

    //The main application loop
    loop {
        let _update_the_model = update(&update_rx, model.clone());
        let _draw_the_window = terminal.draw(|f| draw(f, model.clone()));
        if model.lock().unwrap().should_quit {
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
                )?;
            temp_dir.close()?;
            break;
        }
    }


    Ok(())
}
