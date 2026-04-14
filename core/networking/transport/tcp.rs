use std::io::{Read, Write};
use std::net::TcpStream;

pub fn send_tcp(addr: &str, data: &[u8]) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(addr)?;
    stream.write_all(data)?;
    Ok(())
}

pub fn receive_tcp(stream: &mut TcpStream) -> std::io::Result<Vec<u8>> {
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer)?;
    Ok(buffer)
}
