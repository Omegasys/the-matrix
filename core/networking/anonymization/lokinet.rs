pub fn route_through_lokinet(data: &[u8]) -> Vec<u8> {
    let mut out = b"LOKINET".to_vec();
    out.extend_from_slice(data);
    out
}

pub fn unwrap_lokinet(data: &[u8]) -> Vec<u8> {
    data.get(7..).unwrap_or(&[]).to_vec()
}
