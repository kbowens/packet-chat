use std::thread;
use libc::timeval;
use std::sync::{Arc, Mutex};
use tui::widgets::TableState;
use anyhow;

pub struct Model {
    pub input: String,
    pub current_window: i8,
    pub should_quit: bool,
    pub packets: Arc<Mutex<Vec<Packet>>>,
    packets_to_draw: Vec<Packet>,
    pub search_is_active: bool,
    pub key_mode: KeyMode,
    pub packet_table_state: TableState,
}

impl Model {
    pub fn get_packets_to_draw(&self) -> Option<Vec<Packet>> {
    	if self.search_is_active {
    		return Some(self.packets_to_draw);
    	} else {
	    	let localpacketslist = self.packets.clone();
	    	let pllocked = localpacketslist.lock().unwrap();
	        let length = pllocked.len();
	        match length {
	            0 => return None,
	            1..=20 => return Some(pllocked[0..length].to_vec()),
	            _ => return Some(pllocked[length - 20..length].to_vec()),
	        }
	    }
    }

    pub fn find_keyword(&self, keyword: &str) -> anyhow::Result<()> {
    	let localpacketlist = self.packets.clone();
    	let guard = localpacketlist.lock();
    	let pllocked = guard.unwrap();
    	let len = pllocked.len();
    	std::mem::drop(pllocked);
    	for packet_index in 0..len {
    		let pllocked = localpacketlist.lock().unwrap();
    		let packet_data = pllocked[packet_index].info.data[..].to_vec();
    		let strdata = String::from_utf8(packet_data).unwrap();
    		let s = strdata.find(keyword);
    		match s {
    			Some(index) => {

    			},
    			None => {

    			}
    		}
    	}
    	Ok(())
    }

}

#[derive(Clone)]
pub struct Packet {
    pub header: PacketHeader,
    pub info: PacketInfo,
}

#[derive(Clone)]
pub struct PacketInfo {
    pub eth_dst: Vec<u8>,
    pub eth_src: Vec<u8>,
    pub eth_type: Vec<u8>,
    pub data: Vec<u8>,


}

impl From<Vec<u8>> for PacketInfo {
    fn from(mut newdata: Vec<u8>) -> Self {
        PacketInfo {
        	eth_dst: newdata.drain(0..6).collect(),
        	eth_src: newdata.drain(0..6).collect(),
        	eth_type: newdata.drain(0..2).collect(),
        	data: newdata,

        }
    }
}

#[derive(Clone)]
pub struct PacketHeader {
    pub ts: timeval,
    pub caplen: u32,
    pub len: u32,
}

#[derive(PartialEq)]
pub enum KeyMode {
	Insert,
	Normal,
}


/* Tomorrow, need to clone the pieces of a Packet into my own packet struct
	and then I will be able to place that into a vector that can be sorted through
*/	