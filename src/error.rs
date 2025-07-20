pub enum EDGARParserError {
    HttpError(std::io::Error),
    JSONParseError(serde_json::Error),
    NotFound(String),
    Unauthorized(String),
    InvalidResponse(String),
}
