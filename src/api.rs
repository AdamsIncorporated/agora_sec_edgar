use reqwest::header::USER_AGENT;

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
pub async fn fetch_http_body(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Use custom user agent or fallback
    let user_agent = std::env::var("USER_AGENT")
        .unwrap_or_else(|_| "MyRustApp support@myrustapp.com".to_string());

    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .header(USER_AGENT, user_agent)
        .send()
        .await
        .unwrap_or_else(|e| {
            panic!("HTTP request to {} failed: {}", url, e)
        });
    // debug the values
    println!("DEBUG: GET {} response: {:?}", url, response);
    
    // Check if status is success (200..299)
    if !response.status().is_success() {
        return Err(format!("HTTP request failed: {}", response.status()).into());
    }

    let body = response.text().await?;

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
        let url = "https://example.com/";
        let result = fetch_http_body(url).await;
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
        let url = "https://example.com/nonexistentpage";
        let result = fetch_http_body(url).await;
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
        let url = "https://thisdomaindoesnotexist123456789.com/";
        let result = fetch_http_body(url).await;
        assert!(
            result.is_err(),
            "Expected error for unreachable domain, got: {:?}",
            result
        );
    }

    #[tokio::test]
    async fn test_get_http_response_body_malformed_url() {
        let url = "not a url";
        let result = fetch_http_body(url).await;
        assert!(
            result.is_err(),
            "Expected error for malformed URL input, got: {:?}",
            result
        );
    }

    #[tokio::test]
    async fn test_get_http_response_body_with_query_params() {
        let url = "https://httpbin.org/get?name=test&lang=rust";
        let result = fetch_http_body(url).await;
        assert!(result.is_ok(), "Expected OK response from httpbin.org");
        let body = result.unwrap();
        assert!(
            body.contains("\"name\": \"test\"") || body.contains("httpbin.org"),
            "Expected response body to contain query parameters"
        );
    }
}
