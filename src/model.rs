use std::sync::{Arc, Mutex};
use libc::timeval;

pub struct Model {
    pub input: String,
    pub currentWindow: i8,
    pub should_quit: bool,
    pub packets: Vec<Packet>
}

pub struct Packet {
    pub header: PacketHeader,
    pub data: Vec<u8>,
}

pub struct PacketHeader {
    pub ts: timeval,
    pub caplen: u32,
    pub len: u32,
}

/* Tomorrow, need to clone the pieces of a Packet into my own packet struct
	and then I will be able to place that into a vector that can be sorted through
*/	