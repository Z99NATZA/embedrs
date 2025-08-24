#![allow(dead_code)]

use async_openai::{
    Client,
    config::OpenAIConfig,
    types::{CreateEmbeddingRequestArgs, EmbeddingInput},
};
use crate::app::result::AppResult;

// -----------------------------
// กรณีข้อความเดียว (convenience):
// ภายในเรียก embed_texts เพื่อไม่ให้ logic ซ้ำซ้อน
// -----------------------------
pub async fn embed_text(
    client: &Client<OpenAIConfig>,
    model: &str,
    text: &str,
) -> AppResult<Vec<f32>> {
    let vecs = embed_texts(client, model, &[text.to_string()]).await?;
    Ok(vecs.into_iter().next().unwrap())
}

// -----------------------------
// กรณีหลายข้อความ (batch):
// ใช้ EmbeddingInput::StringArray(Vec<String>)
// -----------------------------
pub async fn embed_texts(
    client: &Client<OpenAIConfig>,
    model: &str,
    texts: &[String],
) -> AppResult<Vec<Vec<f32>>> {
    // แปลง &[String] -> Vec<String> ให้กับ StringArray
    let inputs: Vec<String> = texts.iter().cloned().collect();

    // 1) สร้างคำขอแบบ type-safe
    let req = CreateEmbeddingRequestArgs::default()
        .model(model) // เช่น "text-embedding-3-small"
        .input(EmbeddingInput::StringArray(inputs))
        .build()?;    // ถ้า argument ผิด type/required field ไม่ครบ จะ error ตั้งแต่ตรงนี้

    // 2) เรียก API
    let resp = client.embeddings().create(req).await?;

    // 3) ดึงเวกเตอร์จากทุก item ที่ส่งเข้าไป
    Ok(resp.data.into_iter().map(|d| d.embedding).collect())
}

// -----------------------------
// Test
// -----------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use async_openai::{Client, config::OpenAIConfig};
    use std::env;

    fn has_api_key() -> bool {
        dotenv::dotenv().ok();
        env::var("OPENAI_API_KEY").is_ok() && env::var("OPENAI_EMBED_MODEL").is_ok()
    }

    #[tokio::test]
    #[ignore] // กันไม่ให้รันอัตโนมัติ
    async fn test_embed_text() {
        if !has_api_key() {
            eprintln!("SKIP: env not set");
            return;
        }

        let client: Client<OpenAIConfig> = Client::new();
        let model = env::var("OPENAI_EMBED_MODEL").unwrap();

        let v = embed_text(&client, &model, "hello world")
            .await
            .unwrap();

        assert_eq!(v.len(), 1536);
        assert!(v.iter().all(|x| x.is_finite()));
    }

    #[tokio::test]
    #[ignore] // กันไม่ให้รันอัตโนมัติ
    async fn test_embed_texts_batch() {
        if !has_api_key() {
            eprintln!("SKIP: env not set");
            return;
        }

        let client: Client<OpenAIConfig> = Client::new();
        let model = env::var("OPENAI_EMBED_MODEL").unwrap();

        let vecs = embed_texts(
            &client,
            &model,
            &vec!["anime".into(), "waifu".into()],
        )
        .await
        .unwrap();

        assert_eq!(vecs.len(), 2);
        assert!(vecs.iter().all(|v| v.len() == 1536));
    }
}
