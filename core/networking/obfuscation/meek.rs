pub fn disguise_as_http(data: &[u8]) -> Vec<u8> {
    let mut out = b"POST / HTTP/1.1
Host: example.com

".to_vec();
    out.extend_from_slice(data);
    out
}

pub fn extract(data: &[u8]) -> Vec<u8> {
    if let Some(pos) = data.windows(4).position(|w| w == b"

") {
        data[pos+4..].to_vec()
    } else {
        vec![]
    }
}
