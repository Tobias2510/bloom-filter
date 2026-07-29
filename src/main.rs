use murmur3::murmur3_32;
use std::io::Cursor;

#[derive(Debug)]
struct BloomFilter {
    len: u32,
    hash_count: u32,
    bit_arr: Vec<u8>,
}

impl BloomFilter {
    fn new(item_count: u32, p: f32) -> Self {
        let len = Self::get_len(item_count, p);
        let hash_count = Self::get_hash_count(len as u32, item_count);

        Self {
            len: len as u32,
            hash_count: hash_count,
            bit_arr: vec![0; len],
        }
    }

    fn add(&mut self, value: &str) {
        for i in 0..self.hash_count {
            let hash = murmur3_32(&mut Cursor::new(value), i).unwrap();
            let position = (hash % self.len) as usize;

            self.bit_arr[position] = 1;
        }
    }

    fn check(&self, value: &str) -> bool {
        for i in 0..self.hash_count {
            let hash = murmur3_32(&mut Cursor::new(value), i).unwrap();
            let position = (hash % self.len) as usize;

            if self.bit_arr[position] == 0 {
                return false;
            }
        }

        true
    }

    fn get_len(item_count: u32, p: f32) -> usize {
        let n = item_count as f32;

        let m = -(n * p.ln()) / (2.0_f32.ln().powi(2));
        m.ceil() as usize
    }

    fn get_hash_count(len: u32, item_count: u32) -> u32 {
        if item_count == 0 {
            return 1 as u32;
        }

        let m = len as f32;
        let n = item_count as f32;

        let k = (m / n) * 2_f32.ln();
        k.ceil() as u32
    }
}

fn main() {
    let mut bloom_filter = BloomFilter::new(5, 0.01);
    bloom_filter.add("hello world");

    let is_not_in = bloom_filter.check("abc");
    println!("Is 'abc' in filter? {}", is_not_in);
    // println!("{bloom_filter:?}");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_len() {
        assert_eq!(BloomFilter::get_len(10, 0.05), 63);
        assert_eq!(BloomFilter::get_len(100, 0.05), 624);
        assert_eq!(BloomFilter::get_len(1000, 0.01), 9586);
        assert_eq!(BloomFilter::get_len(5, 0.01), 48);
    }

    #[test]
    fn test_get_hash_count() {
        assert_eq!(BloomFilter::get_hash_count(100, 10), 7);
        assert_eq!(BloomFilter::get_hash_count(1000, 12), 58);
        assert_eq!(BloomFilter::get_hash_count(10, 3), 3);
    }

    #[test]
    fn test_bloom_filter_true() {
        let mut bloom_filter = BloomFilter::new(10, 0.05);
        bloom_filter.add("user1");
        bloom_filter.add("user256");
        bloom_filter.add("john m");

        assert_eq!(bloom_filter.check("user1"), true);
        assert_eq!(bloom_filter.check("user256"), true);
        assert_eq!(bloom_filter.check("john m"), true);
    }

    #[test]
    fn test_bloom_filter_false() {
        let mut bloom_filter = BloomFilter::new(10, 0.05);
        bloom_filter.add("user2");
        bloom_filter.add("bill c");
        bloom_filter.add("jack d");

        assert_eq!(bloom_filter.check("user1"), false);
        assert_eq!(bloom_filter.check("user256"), false);
        assert_eq!(bloom_filter.check("john m"), false);
    }
}
