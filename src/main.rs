extern crate serde;
extern crate serde_json;

use std::io;
use std::net::UdpSocket;

use connection::TagData;
use triangulation::Triangulator;

mod triangulation;
mod connection;

fn main() {
    let anchor_dist = get_anchor_distance();
    let mut triangulator = Triangulator::from(anchor_dist);
    let socket = UdpSocket::bind("0.0.0.0:8888")
        .unwrap();
    
    triangulator.init(); 
    // UDP loop
    loop {
        let mut buf = [0; 256];
        socket.recv_from(&mut buf).unwrap();
        
        let tag_data = match connection::parse_udp(&buf) {
            Ok(v) => v,
            Err(e) => {
                println!("Failed to parse JSON: {}", e);
                continue;
            },
        };
        
        triangulator.update_anchor_info(tag_data);
    }
}

/// Gets the distance between anchors from the user
fn get_anchor_distance() -> f64 {
    // Setting the distance between anchors
    let mut anchor_dist_buf = String::new();
    let anchor_dist: f64;
    
    // Anchor Distance Input Loop
    loop {
        println!("Please Input the distance between the anchors (in meters):");
        io::stdin()
            .read_line(&mut anchor_dist_buf)
            .expect("Failed to read line");
        
        anchor_dist = match anchor_dist_buf.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        break;
    }
    
    anchor_dist
}
