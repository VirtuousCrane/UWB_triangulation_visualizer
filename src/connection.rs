extern crate serde;
extern crate serde_json;

use serde::{Serialize, Deserialize};
use std::error::Error;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
pub struct TagData {
    pub source: String,
    pub range: f64,
}

/// Receives raw data from the UDP connection and parses it into a TagData struct
pub fn parse_udp(buf: &[u8; 256]) -> Result<TagData, Box<dyn Error>> {
   let mut data = str::from_utf8(buf)?;
   data = data.trim_matches(char::from(0));
   data = data.trim();
   
   let parsed_data: TagData = serde_json::from_str(data)?;
   Ok(parsed_data)
}

