pub fn route_through_freenet(data: &[u8]) -> Vec<u8> {
    let mut out = b"FREENET".to_vec();
    out.extend_from_slice(data);
    out
}

pub fn unwrap_freenet(data: &[u8]) -> Vec<u8> {
    data.get(7..).unwrap_or(&[]).to_vec()
}
