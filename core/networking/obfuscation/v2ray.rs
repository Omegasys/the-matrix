pub fn wrap(data: &[u8]) -> Vec<u8> {
    let mut out = b"V2RAY".to_vec();
    out.extend_from_slice(data);
    out
}

pub fn unwrap(data: &[u8]) -> Vec<u8> {
    data.get(5..).unwrap_or(&[]).to_vec()
}
