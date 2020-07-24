use serde::{Deserialize, Serialize};
use serde_json::Result;
use log::{error, warn, info, debug, trace};
use base64::{encode, decode};


pub fn symbol_from_string(data: String) -> Option<Symbol> {
    match serde_json::from_str(&data) {
        Ok(metasymb) => Some(Symbol::Meta(metasymb)),
        Err(_) => Some(Symbol::Content(ContentSymbol::from_str(data)?)),
    }
}

pub enum Symbol {
    Meta(MetaSymbol),
    Content(ContentSymbol),
}

#[derive(Serialize, Deserialize)]
pub struct MetaSymbol {
    ver: u32,
    frames: usize,
    cur_frame: usize,
    content_len: Vec<usize>, // should only have two elements, as per spec v.0
    sha3: String, // should have len==64
    name: String,        
}

pub struct ContentSymbol {
    sequence: u8,
    index: usize,
    data: Vec<u8>,
}

impl ContentSymbol {
    pub fn from_str(data: String) -> Option<Self> {
        let mut iter = data.split("@");
        let num_part = iter.next()?;
        let data_part = iter.next()?;
        let seq = u8::from_str_radix(&num_part[..2], 16).expect("sequence number is not valid hex");
        let ind = usize::from_str_radix(&num_part[2..], 16).expect("index is not valid hex");
        let data = decode(data_part).expect("data invalid");
        Some(ContentSymbol{ sequence: seq, index: ind, data: data })
    }
}
