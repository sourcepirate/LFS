use super::block::{Block, BlockHash};
use super::checksum::CheckSumMap;
use super::checksum::DataBlock;
use std::fmt::Debug;
use std::io::{self, Read, Seek, SeekFrom, Write};

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
                    info!("Trying to find {:?}", sig);
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

pub fn rdiff<T: Read + Seek + Debug>(file_one: &mut T, file_two: &mut T) -> Vec<BlockVal> {
    info!("File One: {:?}, File Two: {:?}", file_one, file_two);
    let checksum = CheckSumMap::from(file_one);
    info!("Signatures: {:?}", &checksum);
    let delta = Delta::new(checksum);
    delta.diff(file_two)
}

pub fn reconstruct<T: Read + Seek + Write>(
    delta: Vec<BlockVal>,
    alpha: &mut T,
    beta: &mut T,
) -> io::Result<()> {
    for del in delta.iter() {
        match del {
            &BlockVal::Number(_id) => {
                let seek_pos: u64 = (_id * BLOCK_SIZE) as u64;
                let mut block_read: DataBlock = [0u8; 512];
                alpha.seek(SeekFrom::Start(seek_pos));
                let result = alpha.read(&mut block_read);
                match result {
                    Ok(0) => {}
                    Ok(_) => {
                        beta.write(&block_read);
                    }
                    Err(_) => {}
                };
            }
            &BlockVal::Chunk(value) => {
                let chk: [u8; 1] = [value];
                beta.write(&chk);
            }
        }
    }

    Ok(())
}
