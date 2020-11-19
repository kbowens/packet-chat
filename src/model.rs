use pcap::Packet;
use std::sync::{Arc, Mutex};

pub struct Model<'a> {
    pub input: String,
    pub currentWindow: i8,
    pub should_quit: bool,

    pub packets: Vec<&'a Packet<'a>>,
}