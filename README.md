# Bloom filter in Rust

This is my simple implementation of a Bloom filter in Rust.

## How to use it

Just import the library and use it like this:

```rust
use bloom_filter::bloom_filter::BloomFilter;

fn main() {
    let mut bloom_filter = BloomFilter::new(10, 0.05);
    bloom_filter.add("test123");
    bloom_filter.add("hello123");
    bloom_filter.add("user123");

    let string1 = "test123";
    let string2 = "hello123";
    let string3 = "user";

    // true
    println!(
        "{string1} is in bloom-filter: {}",
        bloom_filter.check(string1)
    );

    // true
    println!(
        "{string2} is in bloom-filter: {}",
        bloom_filter.check(string2)
    );

    // false
    println!(
        "{string3} is in bloom-filter: {}",
        bloom_filter.check(string3)
    );
}
```

## Tests

In your terminal just run:

```bash
cargo test
```
