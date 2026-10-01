use murmur3::murmur3_32;
use std::io::Cursor;

use crate::bitmap::BitMap;

#[derive(Debug, Eq, PartialEq, Clone, Copy, thiserror::Error)]
pub enum ArgumentsError {
    #[error("item count must be greater than zero")]
    ZeroItems,
    #[error("false positive probability p must be between 0 and 1 (exclusive)")]
    InvalidP,
}

#[derive(Debug)]
pub struct BloomFilter {
    len: u32,
    hash_count: u32,
    bitmap: BitMap,
}

impl BloomFilter {
    pub fn new(item_count: u32, p: f32) -> Result<Self, ArgumentsError> {
        if p.is_nan() {
            return Err(ArgumentsError::InvalidP);
        }

        let len = Self::get_len(item_count, p)?;
        let hash_count = Self::get_hash_count(len as u32, item_count)?;

        Ok(Self {
            len: len as u32,
            hash_count: hash_count,
            bitmap: BitMap::new(len as usize),
        })
    }

    pub fn add(&mut self, value: &str) {
        for i in 0..self.hash_count {
            let hash = murmur3_32(&mut Cursor::new(value), i).unwrap();
            let position = (hash % self.len) as usize;

            match self.bitmap.set(position, true) {
                Ok(_) => (),
                Err(error) => panic!("Error adding a string to the Bloom Filter: {error:?}"),
            }
        }
    }

    pub fn check(&self, value: &str) -> bool {
        for i in 0..self.hash_count {
            let hash = murmur3_32(&mut Cursor::new(value), i).unwrap();
            let position = (hash % self.len) as usize;

            let bit = self.bitmap.get(position);

            match bit {
                Ok(value) => {
                    if value == 0 {
                        return false;
                    }
                }
                Err(error) => panic!("Error checking a string in the Bloom Filter: {error:?}"),
            }
        }
        true
    }

    fn get_len(item_count: u32, p: f32) -> Result<u32, ArgumentsError> {
        if item_count <= 0 {
            return Err(ArgumentsError::ZeroItems);
        }

        if p <= 0.0 || p >= 1.0 {
            return Err(ArgumentsError::InvalidP);
        }

        let n = item_count as f32;

        let m = -(n * p.ln()) / (2.0_f32.ln().powi(2));
        Ok(m.ceil() as u32)
    }

    fn get_hash_count(len: u32, item_count: u32) -> Result<u32, ArgumentsError> {
        if item_count <= 0 {
            return Err(ArgumentsError::ZeroItems);
        }

        let m = len as f32;
        let n = item_count as f32;

        let k = (m / n) * 2_f32.ln();
        Ok(k.ceil() as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::BloomFilter;

    #[test]
    fn test_get_len() {
        assert_eq!(BloomFilter::get_len(10, 0.05).unwrap(), 63);
        assert_eq!(BloomFilter::get_len(100, 0.05).unwrap(), 624);
        assert_eq!(BloomFilter::get_len(1000, 0.01).unwrap(), 9586);
        assert_eq!(BloomFilter::get_len(5, 0.01).unwrap(), 48);
    }

    #[test]
    fn test_get_hash_count() {
        assert_eq!(BloomFilter::get_hash_count(100, 10).unwrap(), 7);
        assert_eq!(BloomFilter::get_hash_count(1000, 12).unwrap(), 58);
        assert_eq!(BloomFilter::get_hash_count(10, 3).unwrap(), 3);
    }

    #[test]
    fn test_bloom_filter_true() {
        let mut bloom_filter = BloomFilter::new(10, 0.05).unwrap();
        bloom_filter.add("user1");
        bloom_filter.add("user256");
        bloom_filter.add("john m");

        assert_eq!(bloom_filter.check("user1"), true);
        assert_eq!(bloom_filter.check("user256"), true);
        assert_eq!(bloom_filter.check("john m"), true);
    }

    #[test]
    fn test_bloom_filter_false() {
        let mut bloom_filter = BloomFilter::new(10, 0.05).unwrap();
        bloom_filter.add("user2");
        bloom_filter.add("bill c");
        bloom_filter.add("jack d");

        assert_eq!(bloom_filter.check("user1"), false);
        assert_eq!(bloom_filter.check("user256"), false);
        assert_eq!(bloom_filter.check("john m"), false);
    }
}
