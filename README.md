# fwon-rs: High-Performance FWON Data Generator for Rust

`fwon-rs` is a highly optimized Rust library for generating data in the proprietary **Fast Write Object Notation (FWON)** format. Designed for **write** speed, it leverages parallel processing and zero-allocation techniques to achieve over 600,000 records per second in generation and I/O.

> Since this project is experimental, it may be unstable and will have bugs.

## Features

- **Extreme Speed:** Utilizes `rayon` for parallel CPU-bound data generation.

- **Minimal Allocation:** Uses `itoa` and `ryu` for efficient integer and floating-point to string conversion.

- **Simple API:** Provides a straightforward function for bulk parallel data creation.

## Usage

To use `fwon-rs` in your project (e.g., in a high-speed data pipeline):

1. **Add to `Cargo.toml`**
```toml
[dependencies]
fwon-rs = "0.1" # Use the latest version
```

2. **Generate Records**

The library's core function generates records in parallel across all available CPU cores.
```rust
use fwon_rs::generator;
use std::io::{self, Write};
use std::fs::File;

fn main() -> io::Result<()> {
    const NUM_RECORDS: u64 = 1_000_000;
    
    // Generates Vec<Vec<u8>> in parallel
    let all_records = generator::generate_records_parallel(NUM_RECORDS);

    // Write to file (I/O is sequential for simplicity)
    let mut file = File::create("output.fwon")?;
    for record_bytes in all_records {
        file.write_all(&record_bytes)?;
    }
    Ok(())
}
```

## Internal Benchmarking (CLI)

For internal testing and benchmarking (using the `src/main.rs` binary), you must enable the `cli` feature:
```bash
# Did not included in the crate, you must clone the repository
cargo run --features="cli" --bin fwon-rs-bench --release -- benchmark.fwon 100000
```

## License

This project is licensed under the **AGPL-3.0-or-later**.