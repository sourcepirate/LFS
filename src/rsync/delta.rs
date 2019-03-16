use super::block::{Block, BlockHash};
use super::checksum::CheckSumMap;
use super::checksum::DataBlock;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

const BLOCK_SIZE: u32 = 512;

#[derive(Debug)]
pub enum BlockVal {
    Number(u32),
    Chunk(u8),
}

struct Delta(CheckSumMap);

impl Delta {
    pub fn new(ck: CheckSumMap) -> Self {
        Delta(ck)
    }

    pub fn diff<T: Read + Seek>(&self, asset: &mut T) -> Vec<BlockVal> {
        let mut vector: Vec<BlockVal> = Vec::new();
        let mut offset: u32 = 0;
        loop {
            let mut block: DataBlock = [0; 512];
            let value = asset.read(&mut block);
            let flag = match value {
                Ok(0) => false,
                Ok(_) => {
                    let blk = Block::new(block.to_vec(), offset as usize);
                    let sig = blk.hash_pair();
                    println!("Trying to find {:?}", sig);
                    let value = self.0.get(sig);
                    match value {
                        Some(idx) => {
                            vector.push(BlockVal::Number(idx));
                            offset += BLOCK_SIZE;
                        }
                        None => {
                            offset += 1;
                            vector.push(BlockVal::Chunk(block[0]));
                            asset.seek(SeekFrom::Start(offset as u64));
                            continue;
                        }
                    };
                    true
                }
                Err(_) => false,
            };
            if !flag {
                break;
            }
        }
        return vector;
    }
}

pub fn rdiff(file_one: &mut File, file_two: &mut File) -> Vec<BlockVal> {
    println!("File One: {:?}, File Two: {:?}", file_one, file_two);
    let checksum = CheckSumMap::from(file_one);
    println!("Signatures: {:?}", &checksum);
    let delta = Delta::new(checksum);
    delta.diff(file_two)
}
