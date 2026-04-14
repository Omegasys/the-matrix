use std::net::UdpSocket;

pub fn send_udp(addr: &str, data: &[u8]) -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.send_to(data, addr)?;
    Ok(())
}

pub fn receive_udp(bind_addr: &str) -> std::io::Result<Vec<u8>> {
    let socket = UdpSocket::bind(bind_addr)?;
    let mut buf = [0u8; 65535];
    let (size, _) = socket.recv_from(&mut buf)?;
    Ok(buf[..size].to_vec())
}
