use async_openai::Client;
use async_openai::config::OpenAIConfig;
use async_openai::types::CreateEmbeddingRequestArgs;
use async_openai::types::EmbeddingInput;
use std::env;

fn has_api_key() -> bool {
    dotenv::dotenv().ok();
    env::var("OPENAI_API_KEY").is_ok() && env::var("OPENAI_EMBED_MODEL").is_ok()
}

#[tokio::test]
#[ignore] // กันไม่ให้รันอัตโนมัติ
async fn embed_returns_correct_dimension_and_finite_values() {
    if !has_api_key() {
        eprintln!("SKIP: env not set");
        return;
    }

    let client: Client<OpenAIConfig> = Client::new();
    let model = env::var("OPENAI_EMBED_MODEL").unwrap();

    let req = CreateEmbeddingRequestArgs::default()
        .model(&model)
        .input(EmbeddingInput::String("ทดสอบ embeddings".to_string()))
        .build()
        .unwrap();

    let resp = client.embeddings().create(req).await.unwrap();
    let v = &resp.data[0].embedding;

    // 1) มิติต้องถูกต้อง
    assert_eq!(v.len(), 1536);

    // 2) ต้องเป็นค่าปกติ (ไม่ NaN/inf)
    assert!(v.iter().all(|x| x.is_finite()));
}