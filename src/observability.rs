use std::time::Instant;
use std::time::Duration;
use tracing::info;

// -----------------------------
// Initialize tracing subscriber with environment filter.
// ถ้ามีการตั้งค่า `RUST_LOG` ใน environment จะใช้ตามนั้น
// ถ้าไม่มี จะใช้ค่า default เป็น `info` level
// -----------------------------
pub fn init_tracing() {
    use tracing_subscriber::{fmt, EnvFilter};
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into())
        )
        .init();
}

// -----------------------------
// Measure execution time of an async future.
// คืนค่าเป็น `(ผลลัพธ์, Duration)`
//
// # Example
// ```rust
// let (result, elapsed) = timeit(some_async_task()).await;
// ```
// -----------------------------
pub async fn timeit<F, T>(fut: F) -> (T, Duration)
where 
    F: std::future::Future<Output = T> 
{
    let t0 = Instant::now();   // เริ่มจับเวลา
    let v = fut.await;         // รอ future ให้เสร็จ
    (v, t0.elapsed())          // คืนผลลัพธ์พร้อมเวลาที่ใช้
}

// -----------------------------
// Peek vector values for debugging/logging.
// แสดงข้อมูลสั้น ๆ ของ vector เช่น ขนาด, ค่า min/max, mean และ 8 ค่าแรก
// -----------------------------
pub fn peek_vec(label: &str, v: &[f32]) {
    // ดึง 8 ค่าแรกมาเก็บไว้ (ถ้ามีน้อยกว่า 8 ก็เอาที่มี)
    let head: Vec<f32> = v.iter().copied().take(8).collect();

    // หา min และ max ของ vector
    let (min, max) = v.iter().fold(
        (f32::INFINITY, f32::NEG_INFINITY), 
        |(mn, mx), &x| (mn.min(x), mx.max(x))
    );

    // คำนวณค่าเฉลี่ย
    let mean = v.iter().sum::<f32>() / v.len() as f32;

    // log ด้วย tracing::info
    info!(
        dim = v.len(),   // จำนวน element
        ?head,           // ค่า 8 ตัวแรก
        min,             // ค่าต่ำสุด
        max,             // ค่าสูงสุด
        mean,            // ค่าเฉลี่ย
        "{label}"        // label สำหรับบอก context
    );
}
