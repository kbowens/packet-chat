use std::sync::mpsc;
use crossterm::{
    event::{Event as CEvent, KeyCode},
};
use crate::model::{Model, KeyMode, Packet};

pub enum Event<I, P> {
    Input(I),
    Tick,
    Traffic(P),
}

pub fn update(rx: &mpsc::Receiver<Event<CEvent, Box<Packet>>>, model: &mut Model) -> anyhow::Result<()> {
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
                        _ => {

                        }
                    }
                }
                if model.key_mode == KeyMode::Normal {
                    match kevent.code {
                        KeyCode::Char('q') => {
                            model.should_quit = true;
                        },
                        KeyCode::Char('i') => {
                            model.key_mode = KeyMode::Insert;
                        },
                        KeyCode::Char('j') => {
                            let i = match model.packet_table_state.selected() {
                                Some(i) => {
                                    let localpacketlist = model.packets.clone();
                                    let lplocked = localpacketlist.lock().unwrap();
                                    if i == lplocked.len() {
                                        model.packet_table_state.select(Some(i));
                                    }else {
                                        model.packet_table_state.select(Some(i+1));
                                    }
                                },
                                None => {
                                    model.packet_table_state.select(Some(0));
                                }
                            };
                        },
                        KeyCode::Char('k') => {
                            let i = match model.packet_table_state.selected() {
                                Some(i) => {
                                    if i == 0 {
                                        model.packet_table_state.select(Some(0));
                                    } else {
                                         model.packet_table_state.select(Some(i-1));
                                    }
                                },
                                None => {
                                    model.packet_table_state.select(Some(0));
                                }
                            };
                        },
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