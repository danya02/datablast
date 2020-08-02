use crate::symbol::{Symbol, MetaSymbol, ContentSymbol};
use std::collections::HashMap;
use sha3::{Digest, Sha3_256};

struct Sequence {
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

impl Sequence {
    pub fn new(meta: MetaSymbol) -> Sequence {
        Sequence { sequence_id: meta.seq_id, file_len: meta.content_len[0], chunks_count: meta.content_len[1], target_hash: meta.get_hash(), file_name: meta.name, chunks: HashMap::new() }
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

