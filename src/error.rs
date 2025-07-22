use thiserror::Error;

#[derive(Debug, Error)]
pub enum EDGARParserError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] std::io::Error),

    #[error("Failed to parse JSON: {0}")]
    JSONParseError(#[from] serde_json::Error),

    #[error("Requested resource not found: {0}")]
    NotFound(String),

    #[error("Unauthorized access: {0}")]
    Unauthorized(String),

    #[error("Received invalid response: {0}")]
    InvalidResponse(String),
}
