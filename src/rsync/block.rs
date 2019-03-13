use adler32::RollingAdler32;
use md5::compute;

pub trait BlockHash {
    fn week_hash(&self) -> u32;
    fn strong_hash(&self) -> Vec<u8>;
}

pub struct Block {
    data: Vec<u8>,
    offset: usize,
}

impl Block {
    fn new(data: Vec<u8>, offset: usize) -> Block {
        Block { data, offset }
    }
}

impl BlockHash for Block {
    fn week_hash(&self) -> u32 {
        let roller = RollingAdler32::from_buffer(&self.data);
        roller.hash()
    }

    fn strong_hash(&self) -> Vec<u8> {
        let digest = compute(&self.data);
        let slice = &digest.0;
        return slice.to_vec();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_hash() {
        let vec: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
        let exp: Vec<u8> = vec![
            209, 90, 229, 57, 49, 136, 15, 215, 183, 36, 221, 120, 136, 180, 180, 237,
        ];
        let block = Block::new(vec, 0);
        assert_eq!(2686992, block.week_hash());
        assert_eq!(exp, block.strong_hash());
    }
}
