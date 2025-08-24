mod app;
mod config;
mod services;
mod observability;

use crate::app::result::AppResult;
use crate::config::openai::build_client;
use crate::services::embeddings::embed_texts;
use crate::config::api_key::ensure_api_key;
use crate::config::api_key::ensure_embed_model;
use tracing::info;
use observability::{init_tracing, timeit, peek_vec};

#[tokio::main]
async fn main() -> AppResult<()> {
    // 1) ตรวจ API key (.env จะถูกโหลดใน ensure_api_key)
    let _ = ensure_api_key()?;
    let model = ensure_embed_model()?;

    // 2) เริ่มระบบ log
    init_tracing();

    // 3) สร้าง OpenAI client
    let client = build_client();

    // 4) ข้อความตัวอย่าง
    let texts = vec![
        "โปรแกรมเมอร์เขียนโค้ด".to_string(),
        "เล่นเกมกับเพื่อน".to_string(),
        "เลื่อนดู tiktok".to_string(),
        "ทำความสะอาดห้องนอน".to_string(),
    ];

    // 5) วัดเวลา + เรียก batch embeddings
    let (vecs, dur) = timeit(embed_texts(&client, &model, &texts)).await;
    let vecs = vecs?;

    info!(
        ms = dur.as_millis() as u64,
        count = vecs.len(),
        "embedding batch done"
    );

    // 6) peek ตัวอย่างให้เห็นภาพ
    peek_vec("sample[0]", &vecs[0]);

    println!("\n\nRunning ok");
    Ok(())
}