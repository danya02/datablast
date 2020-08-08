use crate::symbol::{Symbol, MetaSymbol, ContentSymbol};
use std::collections::HashMap;
use sha3::{Digest, Sha3_256};
use rand::prelude::*;
use std::iter;

struct SequenceDecoder {
    sequence_id: u8,
    chunks: HashMap<usize, Vec<u8>>,
    file_len: usize,
    chunks_count: usize,
    file_name: String,
    target_hash: [u8;32],
}

enum SymbolInsertError {
    WrongSequenceID,
    FileLenMismatch,
    HashMismatch,
    FileNameMismatch,
    ChunkContentMismatch,
}

enum CollectDataError {
    DiscontinuousContentIDs,
    HashMismatch,
}

impl SequenceDecoder {
    pub fn new(meta: MetaSymbol) -> SequenceDecoder {
        SequenceDecoder { sequence_id: meta.seq_id, file_len: meta.content_len[0], chunks_count: meta.content_len[1], target_hash: meta.get_hash(), file_name: meta.name, chunks: HashMap::new() }
    }

    pub fn insert_new(&mut self, symb: Symbol) -> Result<(), SymbolInsertError> {
        match symb {
            Symbol::Meta(meta) => self.insert_meta(meta),
            Symbol::Content(content) => self.insert_content(content),
        }
    }

    fn insert_meta(&self, symb: MetaSymbol) -> Result<(), SymbolInsertError> {
        if symb.content_len[0] != self.file_len { return Err(SymbolInsertError::FileLenMismatch); }
        if symb.seq_id != self.sequence_id { return Err(SymbolInsertError::WrongSequenceID); }
        if symb.get_hash() != self.target_hash { return Err(SymbolInsertError::HashMismatch); }
        if symb.name != self.file_name { return Err(SymbolInsertError::FileNameMismatch); }
        Ok(())
    }

    fn insert_content(&mut self, symb: ContentSymbol) -> Result<(), SymbolInsertError> {
        if symb.sequence != self.sequence_id { return Err(SymbolInsertError::WrongSequenceID); }
        let existing_data = self.chunks.get(&symb.index);
        match existing_data {
            None => {self.chunks.insert(symb.index, symb.data);},
            Some(old_content) => {
                if old_content.len() != symb.data.len() { return Err(SymbolInsertError::ChunkContentMismatch); }
                for (a, b) in old_content.iter().zip(symb.data.iter()) {
                    if a != b { return Err(SymbolInsertError::ChunkContentMismatch); }
                }
            },
        }
        Ok(())
    }

    fn collect_data(&self) -> Result<Vec<u8>, CollectDataError> {
        let mut outp = Vec::new();
        let mut keys = Vec::new();
        for key in self.chunks.keys() {
            keys.push(key);
        }
        keys.sort();
        let mut last_key = 0;
        for key in keys.iter() {
            if *key - last_key > 1 { return Err(CollectDataError::DiscontinuousContentIDs); }
            outp.extend(self.chunks.get(key).unwrap());
        }
        let mut hasher = Sha3_256::new();
        hasher.update(&outp);
        let hash = hasher.finalize();
        if hash.as_slice() != self.target_hash { return Err(CollectDataError::HashMismatch); }
        Ok(outp)
    }
}

struct SequenceEncoderConfig {
    pub persist_each_symbol_for_frames: usize,
    pub max_bytes_per_data_symbol: usize,
    pub data_symbols_between_meta_symbols: usize,
}


impl SequenceEncoderConfig {
    fn new() -> Self { Default::default() }
}

impl Default for SequenceEncoderConfig {
    fn default() -> Self {
        SequenceEncoderConfig {
            persist_each_symbol_for_frames: 1,
            max_bytes_per_data_symbol: 4096,
            data_symbols_between_meta_symbols: 20,
        }
    }
}

struct SequenceEncoder {
    sequence_id: u8,
    data: Vec<u8>,
    config: Option<SequenceEncoderConfig>,
    current_frame: Option<usize>,
    data_index: usize,
    cur_data_chunk_count: usize,
    hash_cached: Option<String>,
}

enum SequenceEncoderConfigSetError {
    AlreadyIterating,
}

impl SequenceEncoder {
    pub fn new<T: AsRef<[u8]>>(data: T) -> SequenceEncoder {
        SequenceEncoder { sequence_id: rand::random(), data: data.as_ref().to_vec(), config: None, current_frame: None, data_index: 0, cur_data_chunk_count: 0, hash_cached: None}
    }

    pub fn set_config(&mut self, config: SequenceEncoderConfig) -> Result<(), SequenceEncoderConfigSetError> {
        if self.current_frame.is_some() { return Err(SequenceEncoderConfigSetError::AlreadyIterating); }
        self.config = Some(config);
        Ok(())
    }

    pub fn new_with_config<T: AsRef<[u8]>>(data: T, config: SequenceEncoderConfig) -> Self {
        let mut enc = Self::new(data);
        enc.set_config(config);
        enc
    }

    pub fn reset_iterator(&mut self) {
         self.current_frame = None;
         self.data_index = 0;
         self.cur_data_chunk_count = 0;
         self.hash_cached = None;
    }
  

    fn total_len(&self) -> usize { unimplemented!(); }
    fn data_chunks_count(&self) -> usize { unimplemented!(); }
    
    fn get_config(&self) -> &SequenceEncoderConfig {
        match &self.config {
            Some(config) => config,
            None => panic!("got to part of code which believes config is present, while it is actually not")
        }
    }

    fn get_next_chunk(&mut self) -> &[u8] {
        let chunk = &self.data[self.data_index ..= self.data_index + self.get_config().max_bytes_per_data_symbol];
        self.data_index += self.get_config().max_bytes_per_data_symbol;
        self.cur_data_chunk_count += 1;
        chunk
    }

    pub fn get_hash(&mut self) -> &str {
        let return_value = match &self.hash_cached {
            Some(hash) => &hash,
            None => {
                let mut hasher = Sha3_256::new();
                hasher.update(&self.data);
                let digest = hasher.finalize();
                &hex::encode(digest)
            }
        };
        self.hash_cached = Some(return_value.to_string());
        return_value
    }
}

impl Iterator for SequenceEncoder {
    type Item = Symbol;
    fn next(&mut self) -> Option<Symbol> {
        if self.config.is_none() { return None; } // TODO: maybe panic instead?
        let conf = self.get_config();
        if self.current_frame.is_none() {
            self.current_frame = Some(0);
        }
        let cur_frame = self.current_frame.unwrap();
        if cur_frame % (conf.data_symbols_between_meta_symbols+1) == 0 {
            let meta = MetaSymbol { ver: 0, seq_id: self.sequence_id, frames: self.total_len(), cur_frame: cur_frame, content_len: vec![self.data.len(), self.data_chunks_count()], sha3: self.get_hash().to_string(), name: "data.bin".to_string() };
            meta.validate();
            Some(Symbol::Meta(meta))
        } else {
            let chunk = self.get_next_chunk();
            let content = ContentSymbol { sequence: self.sequence_id, index: self.cur_data_chunk_count-1, data: chunk.to_vec() };
            Some(Symbol::Content(content))
        }
    }
}

impl ExactSizeIterator for SequenceEncoder {
    fn len(&self) -> usize { unimplemented!(); }
}
