// Imports cho các crates hiệu suất cao
use rand::{Rng, thread_rng};
use std::time::{SystemTime, UNIX_EPOCH};
use itoa;
use ryu;
use rayon::prelude::*;

// Chúng ta định nghĩa một module public tên là `generator`
pub mod generator {
    // Import các dependencies từ bên ngoài module
    use super::*;

    /// Struct này là optional, nhưng giúp tổ chức code và có thể chứa cấu hình sau này
    pub struct FwonRecordGenerator {} 

    impl FwonRecordGenerator {
        /// TẠO (KHÔNG GHI) một bản ghi FWON và trả về Vec<u8>
        /// Hàm này được public để người dùng có thể gọi
        pub fn generate_record_bytes(
            record_id: u64,
            rng: &mut impl Rng,
        ) -> Vec<u8> {
            
            // Ước tính kích thước buffer
            let mut buffer: Vec<u8> = Vec::with_capacity(512);
            
            // Hằng số byte
            const NL: &[u8] = b"\n";
            
            // Lấy timestamp
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs_f64();
            
            let mut now_buffer = ryu::Buffer::new();
            let now_bytes = now_buffer.format(now).as_bytes();
            
            let mut record_id_buffer = itoa::Buffer::new();
            let record_id_bytes = record_id_buffer.format(record_id).as_bytes();

            // --- Bắt đầu xây dựng buffer ---
            buffer.extend_from_slice(NL);
            buffer.extend_from_slice(b"# --- Record ");
            buffer.extend_from_slice(record_id_bytes);
            buffer.extend_from_slice(b" ---");
            buffer.extend_from_slice(NL);
            
            buffer.extend_from_slice(b"UserID=");
            buffer.extend_from_slice(record_id_bytes);
            buffer.extend_from_slice(NL);

            // Tạo username
            let username = random_string(rng, 8); // Gọi hàm helper private
            buffer.extend_from_slice(b"Username=");
            buffer.extend_from_slice(username.as_bytes());
            buffer.extend_from_slice(b"_");
            buffer.extend_from_slice(record_id_bytes);
            buffer.extend_from_slice(NL);

            // Tạo email (Thương hiệu Teaserverse!)
            let email_prefix = random_string(rng, 5);
            buffer.extend_from_slice(b"Email=");
            buffer.extend_from_slice(email_prefix.as_bytes());
            buffer.extend_from_slice(b"@teaserverse.com");
            buffer.extend_from_slice(NL); 

            // Boolean
            buffer.extend_from_slice(b"IsActive=");
            buffer.extend_from_slice(if rng.r#gen() { b"true" } else { b"false" }); // FIX: Dùng r#gen() cho Edition 2024
            buffer.extend_from_slice(NL);

            // Dùng format! cho {:.2}
            let balance = rng.gen_range(0.0..10000.50);
            let balance_str = format!("{:.2}", balance);
            buffer.extend_from_slice(b"Balance=");
            buffer.extend_from_slice(balance_str.as_bytes());
            buffer.extend_from_slice(NL);

            // RYU: Ghi timestamp
            let joined_ts = now - rng.gen_range(0..100000) as f64;
            buffer.extend_from_slice(b"JoinedTimestamp=");
            buffer.extend_from_slice(ryu::Buffer::new().format(joined_ts).as_bytes());
            buffer.extend_from_slice(NL);
            
            // Hằng số (dự án của Teaserverse)
            buffer.extend_from_slice(b"FavoriteProjects=TeaserWorkspace,TeaserPaste,EmmieryAI"); 
            buffer.extend_from_slice(NL);

            // Cài đặt lồng nhau
            buffer.extend_from_slice(b"Settings={{Theme=");
            buffer.extend_from_slice(if rng.r#gen() { b"dark" } else { b"light" }); // FIX: Dùng r#gen() cho Edition 2024
            buffer.extend_from_slice(b";Language=vi;Notifications=true;BetaUser=false}}");
            buffer.extend_from_slice(NL);

            // ITOA + RYU
            let ip1 = rng.gen_range(1..=255);
            buffer.extend_from_slice(b"History=[Action=login;IP=192.168.1.");
            buffer.extend_from_slice(itoa::Buffer::new().format(ip1).as_bytes());
            buffer.extend_from_slice(b";Timestamp=");
            buffer.extend_from_slice(now_bytes); // ryu (tái sử dụng)
            buffer.extend_from_slice(b"]");
            buffer.extend_from_slice(NL);

            // ITOA + RYU
            let ip2 = rng.gen_range(1..=255);
            let ts2 = now + 1.0;
            buffer.extend_from_slice(b"History=[Action=logout;IP=192.168.1.");
            buffer.extend_from_slice(itoa::Buffer::new().format(ip2).as_bytes());
            buffer.extend_from_slice(b";Timestamp=");
            buffer.extend_from_slice(ryu::Buffer::new().format(ts2).as_bytes());
            buffer.extend_from_slice(b"]");
            buffer.extend_from_slice(NL);

            // Thêm 0-2 phiên hoạt động
            let num_sessions = rng.gen_range(0..=2);
            for i in 0..num_sessions {
                let session_id = random_string(rng, 15);
                let session_ts = now + i as f64;
                
                buffer.extend_from_slice(b"Sessions=[SessionID=");
                buffer.extend_from_slice(session_id.as_bytes());
                buffer.extend_from_slice(b";Device=Chrome;Timestamp=");
                buffer.extend_from_slice(ryu::Buffer::new().format(session_ts).as_bytes());
                buffer.extend_from_slice(b"]");
                buffer.extend_from_slice(NL);
            }

            buffer.extend_from_slice(NL); // Dòng trống cuối
            
            buffer // Trả về Vec<u8>
        }
    }

    /// Tạo chuỗi ngẫu nhiên (chỉ chữ thường, giống bản Python)
    fn random_string(rng: &mut impl Rng, len: usize) -> String {
        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
        (0..len)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    /// Một hàm tiện ích (helper) public để tạo song song
    pub fn generate_records_parallel(num_records: u64) -> Vec<Vec<u8>> {
        (0..num_records)
            .into_par_iter()
            .map(|record_id| {
                // Mỗi luồng (thread) của rayon sẽ tự lấy RNG
                let mut rng = thread_rng(); 
                // Gọi hàm tạo byte (CPU-intensive)
                FwonRecordGenerator::generate_record_bytes(record_id, &mut rng)
            })
            .collect() // Thu thập kết quả (có thứ tự)
    }
}