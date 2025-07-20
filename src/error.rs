pub enum EDGARServerError {
    ConnectionError(std::io::Error),
    ParseError(serde_json::Error),
    NotFound(String),
    Unauthorized(String),
    InvalidResponse(String),
}
