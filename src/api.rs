use std::io::{Read, Write};
use std::net::TcpStream;

pub fn get_http_response_body(domain: &str, path: &str) -> Result<String, std::io::Error> {
    let addr_str = format!("{}:80", domain);
    let mut stream = TcpStream::connect(addr_str)?;
    let request = format!(
        "GET {} HTTP/1.1\r\n\
         Host: {}\r\n\
         User-Agent: rust-client/1.0\r\n\
         Connection: close\r\n\
         Accept: application/json\r\n\
         \r\n",
        path, domain
    );

    stream.write_all(request.as_bytes())?;

    // Read the response from the stream
    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    // Split the response into headers and body
    let body = response
        .split("\r\n\r\n")
        .nth(1)
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "No response body found",
        ))?
        .to_string();

    // Check if the body is empty
    if body.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Empty response body",
        ));
    }
    Ok(body)
}
