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
fwon-rs = "0.2.0" # Use the latest version
```

2. **Generate and Write Records (Optimized)**

Use the built-in function `generate_and_write_records_parallel` to handle parallel data generation (CPU-bound) and optimized file writing (Buffered I/O) in a single call:
```rust
use fwon_rs::generator;
use std::io;

fn main() -> io::Result<()> {
    const NUM_RECORDS: u64 = 1_000_000;
    const FILEPATH: &str = "output.fwon";
    
    // Tạo và ghi 1 triệu bản ghi vào file một cách song song và tối ưu I/O.
    let result = generator::generate_and_write_records_parallel(NUM_RECORDS, FILEPATH)?;

    println!("Ghi thành công {} records vào '{}'", NUM_RECORDS, FILEPATH);
    println!("Thời gian tạo dữ liệu (CPU): {:.4}s", result.gen_time_sec);
    println!("Thời gian ghi file (I/O): {:.4}s", result.write_time_sec);
    
    Ok(())
}
```

## Internal Benchmarking (CLI)

For internal testing and benchmarking (using the `src/main.rs` binary), you must enable the `cli` feature:
```bash
# Did not included in the crate, you must clone the repository
cargo run --features="cli" --bin fwon-rs-bench --release -- benchmark.fwon 100000
```

## Speed Demo

This is the benchmark result using a standard SSD disk to write:
```
Using fwon-rs crate to generate and write 100000 FWON records to file 'benchmark.fwon'...

--- FWON WRITE BENCHMARK (Internal) ---
Total records written: 100000
Time (Data Gen CPU):   0.145744 seconds
Time (File Write I/O): 0.051216 seconds
----------------------------------------------
Total Time (Gen + I/O):  0.196961 seconds
WRITE Rate (I/O Only):   1952514.84 records/second
WRITE Rate (Overall):    507715.24 records/second
```

## License

This project is licensed under the **AGPL-3.0-or-later**.