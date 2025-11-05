// Sử dụng các thành phần từ chính thư viện fwon-rs
use fwon_rs::generator;

use clap::Parser;
use std::fs::File;
use std::io::{self, Write, BufWriter};
use std::time::Instant;

/// Script để benchmark tốc độ GHI (Write Speed) của FWON (Parallel: rayon + itoa + ryu)
///
/// Đây là file main.rs nội bộ, chỉ dùng để test thư viện fwon-rs.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filepath để ghi dữ liệu FWON
    #[arg(index = 1)]
    filepath: String,

    /// Số lượng bản ghi cần tạo
    #[arg(index = 2)]
    num_records: u64,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    println!(
        "Sử dụng crate fwon-rs để tạo và ghi {:} bản ghi FWON vào file '{}'...",
        args.num_records, args.filepath
    );

    // --- 1. BENCHMARK TỐC ĐỘ GHI (WRITE BENCHMARK) ---

    // ---- BƯỚC 1: TẠO DỮ LIỆU (CPU-BOUND) ----
    let start_gen_time = Instant::now();

    // Gọi hàm generator::generate_records_parallel từ thư viện fwon-rs
    let all_records: Vec<Vec<u8>> = generator::generate_records_parallel(args.num_records);

    let end_gen_time = Instant::now();

    // ---- BƯỚC 2: GHI DỮ LIỆU (I/O-BOUND) ----
    // Bước này là tuần tự (single-thread), chỉ ghi ra đĩa
    let start_write_time = Instant::now();

    let file = File::create(&args.filepath)?;
    // Sử dụng BufWriter lớn hơn để tối ưu I/O (8MB)
    let mut writer = BufWriter::with_capacity(8 * 1024 * 1024, file);

    for record_bytes in all_records {
        writer.write_all(&record_bytes)?;
    }

    // Quan trọng: Đẩy (flush) tất cả dữ liệu ra đĩa
    writer.flush()?;

    let end_write_time = Instant::now();

    // --- TÍNH TOÁN VÀ IN KẾT QUẢ ---
    let total_gen_time = end_gen_time.duration_since(start_gen_time).as_secs_f64();
    let total_write_time = end_write_time.duration_since(start_write_time).as_secs_f64();
    let total_time = end_write_time.duration_since(start_gen_time).as_secs_f64();
    let records_per_sec_total = args.num_records as f64 / total_time;
    let records_per_sec_io_only = args.num_records as f64 / total_write_time;


    println!("\n--- FWON WRITE BENCHMARK (Nội bộ) ---");
    println!("Tổng số bản ghi đã ghi: {}", args.num_records);
    println!("Thời gian (Tạo data CPU):  {:.6} giây", total_gen_time);
    println!("Thời gian (Ghi file I/O):    {:.6} giây", total_write_time);
    println!("----------------------------------------------");
    println!("Tổng thời gian (Gen + I/O): {:.6} giây", total_time);
    println!("Tốc độ GHI (Chỉ I/O):       {:.2} records/giây", records_per_sec_io_only);
    println!("Tốc độ GHI (Tổng thể):    {:.2} records/giây", records_per_sec_total);

    Ok(())
}