use std::io::{Read, Write};
use std::net::TcpStream;

fn fetch_data_from_server(server: &str, request: &str) -> std::io::Result<String> {
    let mut stream = TcpStream::connect(server)?;
    stream.write_all(request.as_bytes())?;
    
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    
    Ok(response)
}
