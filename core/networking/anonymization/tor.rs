pub fn route_through_tor(data: &[u8]) -> Vec<u8> {
    // Placeholder: simulate onion wrapping
    let mut out = b"TOR".to_vec();
    out.extend_from_slice(data);
    out
}

pub fn unwrap_tor(data: &[u8]) -> Vec<u8> {
    data.get(3..).unwrap_or(&[]).to_vec()
}
