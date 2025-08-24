use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("OPENAI_API_KEY is not set")]
    MissingApiKey,

    #[error("OPENAI_EMBED_MODEL is not set")]
    MissingEmbedModel,

    // รับ error จาก async-openai โดยตรง (จะใช้กับ .await?)
    #[error(transparent)]
    OpenAI(#[from] async_openai::error::OpenAIError),
}