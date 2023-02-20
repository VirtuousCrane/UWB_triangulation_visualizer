extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use std::net::UdpSocket;
use connection::TagData;

mod triangulation;
mod connection;

fn main() {
    let mut anchors: HashMap<String, TagData> = HashMap::new();
    let socket = UdpSocket::bind("0.0.0.0:8888")
        .unwrap();
    
    loop {
        let mut buf = [0; 256];
        socket.recv_from(&mut buf).unwrap();
        
        let data = match connection::parse_udp(&buf) {
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
        
        triangulation::update_position(&anchors);
    }
}
