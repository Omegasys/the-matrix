pub fn route_through_zeronet(data: &[u8]) -> Vec<u8> {
    let mut out = b"ZERONET".to_vec();
    out.extend_from_slice(data);
    out
}

pub fn unwrap_zeronet(data: &[u8]) -> Vec<u8> {
    data.get(8..).unwrap_or(&[]).to_vec()
}
