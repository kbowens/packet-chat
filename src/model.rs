use std::thread;
use libc::timeval;
use std::sync::{Arc, Mutex};
use tui::widgets::TableState;
use anyhow;
use serde::{Serialize, Deserialize};
use chrono::naive::NaiveDateTime;
use pallet::ext::tantivy::schema::Facet;

pub struct Model {
    pub input: String,
    pub current_window: i8,
    pub should_quit: bool,
    pub packets: Arc<Mutex<Vec<Packet>>>,
    pub packets_to_draw: Arc<Mutex<Vec<Packet>>>,
    pub search_is_active: bool,
    pub key_mode: KeyMode,
    pub packet_table_state: TableState,
    pub gauge_ratio: Arc<Mutex<usize>>,
    pub new_packets: Arc<Mutex<Vec<Packet>>>,
    pub selected_packet: Option<Packet>, // Do I need this?
}

impl Model {
    pub fn get_packets_to_draw(&self) -> Option<Vec<Packet>> {
    	if self.search_is_active == true {
    		let arc_ptd = self.packets_to_draw.clone();
    		let access_ptd = arc_ptd.lock().unwrap();
    		return Some(access_ptd.clone());
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

    pub fn select_next_packet(&mut self) {
        let packet_table_size = match self.search_is_active {
            true => {
                self.packets_to_draw.lock().unwrap().len()
            },
            false => {
                //self.packets.lock().unwrap().len()
                //Someday, it will be above. But I have the size capped at 20 rn
                //and I need to figure out how to dynamically change the shown array.
                20
            }
        };
        if packet_table_size == 0 {
            return
        }
        if self.packet_table_state.selected() == None {
            self.packet_table_state.select(Some(0));
        } else {
            let selected_ind = self.packet_table_state.selected().unwrap();
            if selected_ind < packet_table_size - 1 {
                self.packet_table_state.select(Some(selected_ind+1));
            }
        }
        
    }

    pub fn select_previous_packet(&mut self) {
        let packet_table_size = match self.search_is_active {
            true => self.packets_to_draw.lock().unwrap().len(),
            false => self.packets.lock().unwrap().len()
        };
        if packet_table_size == 0 {
            return
        }
        if self.packet_table_state.selected() == None {
            self.packet_table_state.select(Some(0));
        } else {
            let selected_ind = self.packet_table_state.selected().unwrap();
            if selected_ind > 0 {
                self.packet_table_state.select(Some(selected_ind - 1));
            }
        }
    }



    pub fn get_gauge_ratio(&self) -> u16 {
    	let gr_arc = self.gauge_ratio.clone();
    	let gr_lock = gr_arc.lock().unwrap();
    	return *gr_lock as u16;
    }

}

#[derive(Clone, Serialize, Deserialize, Debug, pallet::DocumentLike)]
#[pallet(tree_name = "Packets")]
pub struct Packet {
	#[pallet(skip_indexing)]
    pub header: PacketHeader,
    #[pallet(default_search_field)]
    pub mac_dst: Option<String>,
    #[pallet(default_search_field)]
    pub mac_src: Option<String>,
    #[pallet(default_search_field)]
    pub ip_type: Option<String>,
    #[pallet(default_search_field)]
    pub ip_dst: Option<String>,
    #[pallet(default_search_field)]
    pub ip_src: Option<String>,
    #[pallet(default_search_field)]
    pub payload: String,
}

#[derive(Clone)]
pub struct PacketInfo {
    pub eth_dst: Vec<u8>,
    pub eth_src: Vec<u8>,
    pub eth_type: Vec<u8>,
    pub data: Vec<u8>,


}
//Delete this maybe?
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PacketHeader {
	#[serde(with = "date_format")]
    pub ts: NaiveDateTime,
    pub caplen: u32,
    pub len: u32,
}

mod date_format {
    use chrono::naive::{NaiveDateTime};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &NaiveDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[derive(PartialEq)]
pub enum KeyMode {
	Insert,
	Normal,
}
