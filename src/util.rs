use std::sync::mpsc;
use chrono::naive::NaiveDateTime;
use pcap::{Capture, Activated};
use stfu8;
use crate::model::{Packet, PacketHeader};
use etherparse::{IpHeader};


pub fn process_packets<T: Activated>(cap: &mut Capture<T>, packet_sender: &mpsc::Sender<Box<Packet>>) {
	while let Ok(packet) = cap.next() {
        let newdata = etherparse::PacketHeaders::from_ethernet_slice(&packet);
        let newheader = PacketHeader {
            ts: NaiveDateTime::from_timestamp(packet.header.ts.tv_sec, packet.header.ts.tv_usec as u32),
            caplen: packet.header.caplen,
            len: packet.header.len,
        };
        match newdata {
        	Ok(pheaders) => {
        		let newpacket = Box::new(Packet {
        			header: newheader,
        			mac_dst: if let Some(eth_headers) = &pheaders.link {
        				Some(to_macaddr_string(eth_headers.destination))
        			} else {
        				None
        			},
        			mac_src: if let Some(eth_headers) = &pheaders.link {
        				Some(to_macaddr_string(eth_headers.source))
        			} else {
        				None
        			},
        			ip_type: if let Some(eth_headers) = &pheaders.link {
        				match eth_headers.ether_type {
        					2048 => Some("IPv4".to_string()),
        					2054 => Some("ARP".to_string()),
        					34525 => Some("IPv6".to_string()),
        					_ => None,
        				}
        			} else {
        				None
        			},
        			payload: stfu8::encode_u8(pheaders.payload),
        			ip_dst: if let Some(ip_headers) = &pheaders.ip {
        				match ip_headers {
        					IpHeader::Version4(header) => Some(to_ip4addr_string(header.destination)),
        					IpHeader::Version6(header) => Some(to_ip6addr_string(header.destination)),
        				}
        			} else {
        				None
        			},
        			ip_src: if let Some(ip_headers) = &pheaders.ip {
        				match ip_headers {
        					IpHeader::Version4(header) => Some(to_ip4addr_string(header.source)),
        					IpHeader::Version6(header) => Some(to_ip6addr_string(header.source)),
        				}
        			} else {
        				None
        			}
        			
        		});
        		let _send_packet = packet_sender.send(newpacket);
        	},
        	_ => {

        	}
        }
    }
}

fn to_macaddr_string(addr: [u8; 6]) -> std::string::String {
	let string: std::string::String = format!("{:x}:{:x}:{:x}:{:x}:{:x}:{:x}",addr[0], addr[1], addr[2], addr[3], addr[4], addr[5]);

	return string;
}

fn to_ip4addr_string(addr: [u8; 4]) -> std::string::String {
	let string = format!("{}.{}.{}.{}", addr[0], addr[1], addr[2], addr[3]);
	return string;
}

fn to_ip6addr_string(addr: [u8; 16]) -> std::string::String {
	let string = format!("{:x}{:x}:{:x}{:x}:{:x}{:x}:{:x}{:x}",
							addr[0], addr[1], addr[2], addr[3], addr[4], addr[5],
							addr[6], addr[7]);
	return string;
}

