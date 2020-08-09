use serde::{Deserialize, Serialize};
use serde_json;
use log::{error, warn, info, debug, trace};
use base64::{encode, decode, DecodeError};
use core::num::ParseIntError;
use hex;
use thiserror::Error;

pub type Version = u32;

#[derive(Debug, Eq, PartialEq, Error)]
pub enum SymbolDecodeError {
    #[error("There was an error while decoding this content symbol: {0}")]
    InvalidContent(ContentDecodeError),
    #[error("There was an error while decoding this meta symbol: {0}")]
    InvalidMeta(MetaDecodeError),
}

#[derive(Debug, Eq, PartialEq, Error)]
pub enum ContentDecodeError {
    #[error("The data part was not valid Base64: {0}")]
    InvalidDataPart(DecodeError),

    #[error("The sequence ID was not valid hex: {0}")]
    InvalidSequenceIdPart(ParseIntError),

    #[error("The piece ID was not valid hex: {0}")]
    InvalidPieceIdPart(ParseIntError),

    #[error("The data part was empty")]
    NoDataPart
}

#[derive(Debug, Eq, PartialEq, Error)]
pub enum MetaDecodeError {
    #[error("This program version does not understand this meta version: {0}")]
    UnknownVersion(Version),

    #[error("There were {0} elements in the content_len array while 2 were expected")]
    InvalidLengthOfContentLen(usize),

    #[error("The hash field was supposed to be a string of 64 characters, but {0} were found instead")]
    InvalidLengthOfHashField(usize),

    #[error("The hash field is not a valid hex number")]
    HashFieldNotHex,
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

#[derive(Debug, Eq, PartialEq)]
/// Some sort of symbol. Currently supported are meta symbols and content symbols.
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
#[derive(Debug, Eq, PartialEq)]
/// A meta symbol. Contains information about the sequence.
pub struct MetaSymbol {
    pub ver: Version,
    pub seq_id: u8,
    pub frames: usize,
    pub cur_frame: usize,
    pub content_len: Vec<usize>, // should only have two elements, as per spec v.0
    pub sha3: String, // should have len==64
    pub name: String,
}

impl MetaSymbol {
    pub fn validate(&self) -> MetaValidateResult {
        if self.ver != 0 {return Err(MetaDecodeError::UnknownVersion(self.ver));}
        if self.content_len.len() != 2 {return Err(MetaDecodeError::InvalidLengthOfContentLen(self.content_len.len()));}
        if self.sha3.len() != 64 {return Err(MetaDecodeError::InvalidLengthOfHashField(self.sha3.len()));}
        let decode_res = hex::decode(&self.sha3);
        if decode_res.is_err() { return Err(MetaDecodeError::HashFieldNotHex); }
        Ok(())
    }

    pub fn to_str(&self) -> String { serde_json::to_string(self).expect("JSON serialization failed?!") }
    pub fn get_hash(&self) -> [u8;32] {
        let vec = hex::decode(&self.sha3).unwrap(); // this should not panic if this has been validated
        let slice = vec.as_slice();
        let mut array = [0; 32];
        array.copy_from_slice(slice);
        array
    }
}

#[derive(Debug, Eq, PartialEq)]
/// A content symbol. Contains a piece of data from the encoded file.
pub struct ContentSymbol {
    pub sequence: u8,
    pub index: usize,
    pub data: Vec<u8>,
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
        format!("{:02x}{:x}@{}", self.sequence, self.index, encode(&self.data))
    }
}
