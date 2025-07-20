use serde::Deserialize;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Debug, Deserialize)]
pub struct CompanyTicker {
    pub cik_str: u32,
    pub ticker: String,
    pub title: String,
}

pub fn fetch_cik_json_from_server() -> std::io::Result<String> {
    let mut stream = TcpStream::connect("www.sec.gov:80")?;

    let request = "\
    GET /files/company_tickers.json HTTP/1.1\r\n\
    Host: www.sec.gov\r\n\
    User-Agent: rust-client/1.0\r\n\
    Connection: close\r\n\
    Accept: application/json\r\n\
    \r\n";

    stream.write_all(request.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    let body = response
        .split("\r\n\r\n")
        .nth(1)
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "No response body found",
        ))?;

    let parsed: HashMap<String, CompanyTicker> = serde_json::from_str(body)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    Ok(parsed)
}

pub fn parse_company_tickers(
    json_str: &str,
) -> Result<HashMap<String, CompanyTicker>, serde_json::Error> {
    serde_json::from_str(json_str)
}
