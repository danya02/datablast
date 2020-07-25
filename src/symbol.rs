use serde::{Deserialize, Serialize};
use serde_json;
use log::{error, warn, info, debug, trace};
use base64::{encode, decode, DecodeError};
use core::num::ParseIntError;

pub type Version = u32;

#[derive(Debug)]
pub enum SymbolDecodeError {
    InvalidContent(ContentDecodeError),
    InvalidMeta(MetaDecodeError),
}

#[derive(Debug)]
pub enum ContentDecodeError {
    InvalidDataPart(DecodeError),
    InvalidSequenceIdPart(ParseIntError),
    InvalidPieceIdPart(ParseIntError),
    NoDataPart
}

#[derive(Debug)]
pub enum MetaDecodeError {
    UnknownVersion(Version),
    InvalidLengthOfContentLen(usize),
    InvalidLengthOfHashField(usize),
}

pub type MetaValidateResult = Result<(), MetaDecodeError>;

pub type SymbolDecodeResult = Result<Symbol, SymbolDecodeError>;


pub fn symbol_from_string(data: String) -> SymbolDecodeResult {
    match serde_json::from_str(&data) {
        Ok(metasymb) => {
                            let metasymb: MetaSymbol = metasymb;
                            match metasymb.validate() {
                                     Ok(_) => Ok(Symbol::Meta(metasymb)),
                                     Err(error) => Err(SymbolDecodeError::InvalidMeta(error)),
                                }
                        },
        Err(error) => {
            trace!("Couldn't decode symbol as JSON: {:?} (data is {:?})", error, data);
            match ContentSymbol::from_str(data) {
                Ok(contentsymb) => Ok(Symbol::Content(contentsymb)),
                Err(error) => Err(SymbolDecodeError::InvalidContent(error))
            }
        }
    }
}

#[derive(Debug)]
pub enum Symbol {
    Meta(MetaSymbol),
    Content(ContentSymbol),
}

impl Symbol {
    pub fn to_str(&self) -> String {
        match self {
            Symbol::Meta(symb) => symb.to_str(),
            Symbol::Content(symb) => symb.to_str(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct MetaSymbol {
    ver: Version,
    frames: usize,
    cur_frame: usize,
    content_len: Vec<usize>, // should only have two elements, as per spec v.0
    sha3: String, // should have len==64
    name: String,
}

impl MetaSymbol {
    pub fn validate(&self) -> MetaValidateResult {
        if self.ver != 0 {return Err(MetaDecodeError::UnknownVersion(self.ver));}
        if self.content_len.len() != 2 {return Err(MetaDecodeError::InvalidLengthOfContentLen(self.content_len.len()));}
        if self.sha3.len() != 64 {return Err(MetaDecodeError::InvalidLengthOfHashField(self.sha3.len()));}
        Ok(())
    }

    pub fn to_str(&self) -> String { serde_json::to_string(self).expect("JSON serialization failed?!") }
}

#[derive(Debug)]
pub struct ContentSymbol {
    sequence: u8,
    index: usize,
    data: Vec<u8>,
}

impl ContentSymbol {
    pub fn from_str(data: String) -> Result<Self, ContentDecodeError> {
        let mut iter = data.split("@");
        let num_part = iter.next().expect("Wasn't able to get first element in a string split by character?!!");
        let data_part;
        match iter.next(){
            Some(data) => {data_part = data;},
            None => {return Err(ContentDecodeError::NoDataPart);},
        }
        let seq = match u8::from_str_radix(&num_part[..2], 16) { Ok(val)=>val, Err(error)=>{return Err(ContentDecodeError::InvalidSequenceIdPart(error));} };
        let ind = match usize::from_str_radix(&num_part[2..], 16) { Ok(val)=>val, Err(error)=>{return Err(ContentDecodeError::InvalidPieceIdPart(error));} };
        let data = match decode(data_part) {Ok(data)=>data, Err(error)=>{return Err(ContentDecodeError::InvalidDataPart(error)); } };
        Ok(ContentSymbol{ sequence: seq, index: ind, data: data })
    }
    pub fn to_str(&self) -> String {
        format!("{:02x}{:x}@{}", self.sequence, self.index, encode(self.data))
    }
}
