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
                        }
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
            model.packets.push(*packet);
        }
    }

    Ok(())
}