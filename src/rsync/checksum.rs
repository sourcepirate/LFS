//! Uses checksum to calculate block checksums
//! from fragments on files.

use super::block::{Block, BlockHash, Signature};
use std::io::Read;

pub type DataBlock = [u8; 512];

#[derive(Debug)]
pub struct CheckSumMap {
    inner: Vec<Signature>,
    lens: u32,
}

impl CheckSumMap {
    pub fn new() -> Self {
        CheckSumMap {
            inner: Vec::new(),
            lens: 0,
        }
    }

    pub fn contains(&self, sig: Signature) -> bool {
        self.inner.contains(&sig)
    }

    pub fn get(&self, sig: Signature) -> Option<u32> {
        for (i, item) in self.inner.iter().enumerate() {
            if item.eq(&sig) {
                return Some(i as u32);
            }
        }
        None
    }

    pub fn len(&self) -> u32 {
        self.lens
    }

    pub fn from<T: Read>(device: &mut T) -> Self {
        let mut vector: Vec<Signature> = Vec::new();
        let mut offset: u32 = 0;
        loop {
            let mut block: DataBlock = [0; 512];
            let result = device.read(&mut block);

            let flag = match result {
                Ok(0) => false,
                Ok(_) => {
                    let data_block = Block::new(block.to_vec(), offset as usize);
                    vector.push(data_block.hash_pair());
                    offset += 1;
                    true
                }
                Err(_) => false,
            };
            if !flag {
                break;
            }
        }

        CheckSumMap {
            inner: vector,
            lens: offset,
        }
    }
}
