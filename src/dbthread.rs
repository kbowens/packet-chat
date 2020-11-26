use std::sync::{Arc, Mutex};
use crate::model::{Model, Packet};
use pallet;


pub fn handle_db(model: Arc<Mutex<Model>>, packets_to_insert: Arc<Mutex<Vec<Packet>>>) {
	//the shared vector here could really be a channel instead, might be more efficient.

	//Create Pallet Database
	//TODO: where to delete this temp dir?
    let temp_dir = tempfile::TempDir::new_in(".").unwrap();
    let db = pallet::ext::sled::open(temp_dir.path().join("db")).unwrap();
    let packetstore: pallet::Store<Packet> = pallet::Store::builder()
        .with_db(db)
        .with_index_dir(temp_dir.path())
        .finish().unwrap();

	loop {
		let mut packetlock = packets_to_insert.lock().unwrap();
		if !packetlock.is_empty() {
			let new_packets: Vec<Packet> = packetlock.to_vec();
			packetlock.clear();
			std::mem::drop(packetlock);
			packetstore.create_multi(&new_packets);
			//packetstore.index_all();
		} else {
			std::mem::drop(packetlock);
		}
		let mut modellock = model.lock().unwrap();
		if modellock.search_is_active && !modellock.input.is_empty() {
			let query = modellock.input.clone();
			modellock.input = "".to_string();
			std::mem::drop(modellock);
			let result = packetstore.search(query.as_str()).unwrap();
			match result {
				pallet::search::Results { count, hits: _ } => {
					let mut result_packets: Vec<Packet> = vec![];
					for id in 0..count {
						let found: Option<pallet::Document<Packet>> = packetstore.find(id as u64).unwrap();
						let found_document: pallet::Document<Packet> = found.unwrap();
						let found_packet: Packet = found_document.inner;
						result_packets.push(found_packet);
					}
					let arc_model = model.clone();
					let locked_model = arc_model.lock().unwrap();
					let draw_search_results: Arc<Mutex<Vec<Packet>>> = locked_model.packets_to_draw.clone();
					let mut locked_draw_search_results = draw_search_results.lock().unwrap();
					locked_draw_search_results.clear();
					locked_draw_search_results.append(&mut result_packets);//clears result_packets
					//locked_model.input = "search complete".to_string();
				},
			}
		}
	}
}