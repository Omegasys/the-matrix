pub fn wrap_soft(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len() + 2);
    out.push(0x13);
    out.extend_from_slice(data);
    out.push(0x37);
    out
}

pub fn unwrap_soft(data: &[u8]) -> Vec<u8> {
    if data.len() >= 2 {
        data[1..data.len()-1].to_vec()
    } else {
        vec![]
    }
