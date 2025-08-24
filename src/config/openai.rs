#![allow(dead_code)]

use async_openai::Client;
use async_openai::config::OpenAIConfig;

// -----------------------------
// คืนค่า OpenAI client ที่พร้อมใช้งาน
// -----------------------------
pub fn build_client() -> Client<OpenAIConfig> {
    Client::new()
}
