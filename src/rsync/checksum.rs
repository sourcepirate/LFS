//! Uses checksum to calculate block checksums
//! from fragments on files.

use super::block::{Block, BlockHash};
use std::collections::BTreeMap;
use std::io::Read;

pub type DataBlock = [u8; 4096];

pub struct CheckSumMap {
    tree: BTreeMap<u32, Vec<Vec<u8>>>,
}

impl CheckSumMap {
    pub fn new() -> Self {
        CheckSumMap {
            tree: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, k: u32, val: Vec<u8>) {
        match self.tree.get_mut(&k) {
            Some(value) => value.push(val),
            None => {
                let mut vector = Vec::new();
                vector.push(val);
                self.tree.insert(k, vector);
            }
        }
    }

    pub fn contains<T: BlockHash>(&self, block: T) -> bool {
        let (week, strong) = block.hash_pair();
        match self.tree.get(&week) {
            Some(value) => value.contains(&strong),
            None => false,
        }
    }

    pub fn get<T: BlockHash>(&self, block: T) -> Option<&Vec<Vec<u8>>> {
        self.tree.get(&block.week_hash())
    }

    pub fn checksums(&self) -> &BTreeMap<u32, Vec<Vec<u8>>> {
        return &self.tree;
    }
}

pub struct CheckSumReader<T: Read> {
    inner: T,
}

impl<T: Read> CheckSumReader<T> {
    pub fn new(inner: T) -> Self {
        CheckSumReader { inner }
    }

    pub fn digest(&mut self) -> CheckSumMap {
        let mut treemap: CheckSumMap = CheckSumMap::new();
        let mut offset: usize = 0;
        loop {
            let mut bytes: DataBlock = [0u8; 4096];
            let result = self.inner.read(&mut bytes);
            match result {
                Ok(_) => {
                    let block = Block::new(bytes.to_vec(), offset);
                    treemap.insert(block.week_hash(), block.strong_hash());
                    offset += 1;
                }
                Err(_) => return treemap,
            }
        }
        return treemap;
    }
}
