use std::sync::mpsc;
use std::thread;
use std::sync::{Arc, Mutex};
use crossterm::{
    event::{Event as CEvent, KeyCode},
};
use crate::model::{Model, KeyMode, Packet};
use crate::search::search;
use pallet;

use crate::model;

pub enum Event<I, P> {
    Input(I),
    Tick,
    Traffic(P),
}

pub fn update(rx: &mpsc::Receiver<Event<CEvent, Box<Packet>>>, guarded_model: Arc<Mutex<Model>>) -> anyhow::Result<()> {
    let mut model = guarded_model.lock().unwrap();
    match rx.recv()? {
        Event::Input(event) => match event {
            CEvent::Key(kevent) => {
                if model.key_mode == KeyMode::Insert {
                    match kevent.code {
                        KeyCode::Char(c) => {
                            model.input.push(c);
                        },
                        KeyCode::Esc => {
                            model.key_mode = KeyMode::Normal;
                        },
                        KeyCode::Backspace => {
                            model.input.pop();
                        },
                        KeyCode::Enter => {
                            if !model.input.is_empty() {
                                model.search_is_active = true;
                                model.key_mode = KeyMode::Normal;
                                model.packet_table_state.select(None);
                            }
                        },
                        _ => {

                        }
                    };
                }
                if model.key_mode == KeyMode::Normal {
                    match kevent.code {
                        KeyCode::Char('q') => {
                            model.should_quit = true;
                        },
                        KeyCode::Char('i') => {
                            model.key_mode = KeyMode::Insert;
                            model.packet_table_state.select(None);
                        },
                        KeyCode::Char('j') => {
                            model.select_next_packet();
                        },
                        KeyCode::Char('k') => {
                            model.select_previous_packet();
                        },
                        KeyCode::Char('c') => {
                            model.input.clear();
                            model.search_is_active = false;
                            model.packet_table_state.select(None);
                        }
                        _ => {

                        }
                    }
                }
                
            }, 
            CEvent::Mouse(mevent) => {

            },
            CEvent::Resize(rh, rw) => {

            }
        },
        Event::Tick => {

        }, 
        Event::Traffic(packet) => {
            let localpacketlist = model.packets.clone();
            let mut unlocked = localpacketlist.lock().unwrap();
            unlocked.push(*packet);
        }
    }

    Ok(())
}