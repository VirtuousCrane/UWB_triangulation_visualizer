extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use std::{net::UdpSocket, error::Error};
use std::str::{self, Utf8Error};

use serde::{Serialize, Deserialize};

mod triangulation;

#[derive(Serialize, Deserialize, Debug)]
struct TagData {
    source: String,
    range: f64,
}

/// Receives raw data from the UDP connection and parses it into a TagData struct
fn parse_udp(buf: &[u8; 256]) -> Result<TagData, Box<dyn Error>> {
   let mut data = str::from_utf8(buf)?;
   data = data.trim_matches(char::from(0));
   data = data.trim();
   
   let parsed_data: TagData = serde_json::from_str(data)?;
   Ok(parsed_data)
}

fn main() {
    let mut anchors: HashMap<String, TagData> = HashMap::new();
    let socket = UdpSocket::bind("0.0.0.0:8888")
        .unwrap();
    
    loop {
        let mut buf = [0; 256];
        socket.recv_from(&mut buf).unwrap();
        
        let data = match parse_udp(&buf) {
            Ok(v) => v,
            Err(e) => {
                println!("Failed to parse JSON: {}", e);
                continue;
            },
        };
        let data_source = String::from(&data.source);
        
        if let None = anchors.insert(data_source.clone(), data) {
            println!("Added new anchor: {}", data_source.clone());
        }
        
        triangulation::update_position();
    }
}
