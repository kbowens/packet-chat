use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use crossterm::event::Event as CEvent;
use crate::model::{Packet};
use crate::update::Event;

pub fn handle_packets(packet_receiver: &mpsc::Receiver<Box<Packet>>,
		update_tx: &mpsc::Sender<Event<CEvent,
		Box<Packet>>>, 
		pthread_packet_list: Arc<Mutex<Vec<Packet>>>) {

	loop {
		if let Ok(new_packet) = packet_receiver.try_recv() { 
			//let packets: Vec<model::Packet> = (&model.packets_to_draw.lock().unwrap()).to_vec();
			//models_list.push(new_packet.clone());
			pthread_packet_list.lock().unwrap().push(*new_packet.clone());
			update_tx.send(Event::Traffic(new_packet));
		}
		else {
			thread::sleep(Duration::from_millis(10))
		}
	}
}