use crate::app::error::AppError;
use crate::app::result::AppResult;

// -----------------------------
// เช็ค api key
// -----------------------------
pub fn ensure_api_key() -> AppResult<String> {
    // โหลดไฟล์ .env ถ้ามี (จะไม่ error ถ้าไม่มี)
    dotenv::dotenv().ok();

    // อ่านจาก environment
    std::env::var("OPENAI_API_KEY").map_err(|_| AppError::MissingApiKey)
}

// -----------------------------
// เช็ค embed model
// -----------------------------
pub fn ensure_embed_model() -> AppResult<String> {
    dotenv::dotenv().ok();
    std::env::var("OPENAI_EMBED_MODEL").map_err(|_| AppError::MissingEmbedModel)
}