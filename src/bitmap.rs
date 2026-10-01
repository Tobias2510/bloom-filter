#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum BitMapError {
    #[error("invalid index")]
    InvalidIndex,
}

#[derive(Debug)]
pub struct BitMap {
    items: Vec<u8>,
    length: usize,
}

impl BitMap {
    pub fn new(length: usize) -> Self {
        BitMap {
            items: vec![0; (length as usize).div_ceil(8)],
            length,
        }
    }

    pub fn set(&mut self, index: usize, bit: bool) -> Result<(), BitMapError> {
        if index >= self.length {
            return Err(BitMapError::InvalidIndex);
        }

        let pos = index / 8;
        let offset = index % 8;

        match bit {
            true => self.items[pos] |= 1u8 << offset,
            false => self.items[pos] &= !(1u8 << offset),
        }

        Ok(())
    }

    pub fn get(&self, index: usize) -> Result<u8, BitMapError> {
        if index >= self.length {
            return Err(BitMapError::InvalidIndex);
        }

        let pos = index / 8;
        let offset = index % 8;

        Ok(((self.items[pos] & (1u8 << offset)) != 0) as u8)
    }
}
