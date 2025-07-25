use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use url::Url;

/// Creates and returns a client capable of making requests to the EDGAR system.
/// Ensure you set the `USER_AGENT` environment variable beforehand.
/// [Per SEC guidelines](https://www.sec.gov/os/webmaster-faq#developers), the `USER_AGENT` should follow this format:
/// ```txt
/// Your Company Name AdminContact@yourcompanydomain.com
/// ```
/// In Rust projects, it’s recommended to define this in [`/your_project/.cargo/config.toml`](https://doc.rust-lang.org/cargo/reference/config.html#hierarchical-structure).
///
/// Example:
/// ```
pub async fn get_http_response_body(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let parsed_url = Url::parse(url)?;
    let host = parsed_url.host_str().ok_or("Invalid host")?;
    let port = parsed_url.port_or_known_default().unwrap_or(80);
    let user_agent = std::env::var("USER_AGENT").unwrap_or_else(|_| "MyRustApp support@myrustapp.com".to_string());
    let path = parsed_url.path().to_string()
        + parsed_url
            .query()
            .map(|q| format!("?{}", q))
            .as_deref()
            .unwrap_or("");

    let addr = format!("{}:{}", host, port);
    let mut stream = TcpStream::connect(addr).await?;

    let request = format!(
        "GET {} HTTP/1.1\r\n\
         Host: {}\r\n\
         User-Agent: {}\r\n\
         Connection: close\r\n\
         Accept: */*\r\n\
         \r\n",
        path, host, user_agent
    );

    stream.write_all(request.as_bytes()).await?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response).await?;

    let response_str = String::from_utf8_lossy(&response);
    let body = response_str
        .split("\r\n\r\n")
        .nth(1)
        .ok_or("No response body found")?
        .to_string();

    if body.is_empty() {
        Err("Empty response body".into())
    } else {
        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_http_response_body_valid_url() {
        let url = "http://example.com/";
        let result = get_http_response_body(url).await;
        assert!(
            result.is_ok(),
            "Expected OK response from example.com, got: {:?}",
            result
        );
        let body = result.unwrap();
        assert!(
            body.contains("Example Domain") || body.contains("<html>"),
            "Expected body to contain expected HTML content, got: {}",
            body
        );
    }

    #[tokio::test]
    async fn test_get_http_response_body_404() {
        let url = "http://example.com/nonexistentpage";
        let result = get_http_response_body(url).await;
        assert!(
            result.is_ok(),
            "Expected valid HTTP response even for 404 page"
        );
        let body = result.unwrap();
        assert!(
            !body.is_empty(),
            "Expected non-empty response body even for a 404 page"
        );
    }

    #[tokio::test]
    async fn test_get_http_response_body_invalid_domain() {
        let url = "http://thisdomaindoesnotexist123456789.com/";
        let result = get_http_response_body(url).await;
        assert!(
            result.is_err(),
            "Expected error for unreachable domain, got: {:?}",
            result
        );
    }

    #[tokio::test]
    async fn test_get_http_response_body_malformed_url() {
        let url = "not a url";
        let result = get_http_response_body(url).await;
        assert!(
            result.is_err(),
            "Expected error for malformed URL input, got: {:?}",
            result
        );
    }

    #[tokio::test]
    async fn test_get_http_response_body_with_query_params() {
        let url = "http://httpbin.org/get?name=test&lang=rust";
        let result = get_http_response_body(url).await;
        assert!(result.is_ok(), "Expected OK response from httpbin.org");
        let body = result.unwrap();
        assert!(
            body.contains("\"name\": \"test\"") || body.contains("httpbin.org"),
            "Expected response body to contain query parameters"
        );
    }
}
