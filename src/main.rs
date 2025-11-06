// Uses components from the fwon-rs library itself
use fwon_rs::generator;

use clap::Parser;
use std::fs::File;
use std::io::{self, Write, BufWriter};
use std::time::Instant;

/// Script to benchmark the WRITE Speed of FWON (Parallel: rayon + itoa + ryu)
///
/// This is an internal main.rs file, only used for testing the fwon-rs library.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filepath to write the FWON data
    #[arg(index = 1)]
    filepath: String,

    /// Number of records to generate
    #[arg(index = 2)]
    num_records: u64,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    println!(
        "Using fwon-rs crate to generate and write {:} FWON records to file '{}'...",
        args.num_records, args.filepath
    );

    // --- 1. WRITE SPEED BENCHMARK ---

    // ---- STEP 1: DATA GENERATION (CPU-BOUND) ----
    let start_gen_time = Instant::now();

    // Calls the generator::generate_records_parallel function from the fwon-rs library
    let all_records: Vec<Vec<u8>> = generator::generate_records_parallel(args.num_records);

    let end_gen_time = Instant::now();

    // ---- STEP 2: DATA WRITING (I/O-BOUND) ----
    // This step is sequential (single-thread), only writing to disk
    let start_write_time = Instant::now();

    let file = File::create(&args.filepath)?;
    // Use a larger BufWriter for I/O optimization (8MB)
    let mut writer = BufWriter::with_capacity(8 * 1024 * 1024, file);

    for record_bytes in all_records {
        writer.write_all(&record_bytes)?;
    }

    // Important: Flush all data to disk
    writer.flush()?;

    let end_write_time = Instant::now();

    // --- CALCULATION AND RESULT PRINTING ---
    let total_gen_time = end_gen_time.duration_since(start_gen_time).as_secs_f64();
    let total_write_time = end_write_time.duration_since(start_write_time).as_secs_f64();
    let total_time = end_write_time.duration_since(start_gen_time).as_secs_f64();
    let records_per_sec_total = args.num_records as f64 / total_time;
    let records_per_sec_io_only = args.num_records as f64 / total_write_time;


    println!("\n--- FWON WRITE BENCHMARK (Internal) ---");
    println!("Total records written: {}", args.num_records);
    println!("Time (Data Gen CPU):   {:.6} seconds", total_gen_time);
    println!("Time (File Write I/O): {:.6} seconds", total_write_time);
    println!("----------------------------------------------");
    println!("Total Time (Gen + I/O):  {:.6} seconds", total_time);
    println!("WRITE Rate (I/O Only):   {:.2} records/second", records_per_sec_io_only);
    println!("WRITE Rate (Overall):    {:.2} records/second", records_per_sec_total);

    Ok(())
}