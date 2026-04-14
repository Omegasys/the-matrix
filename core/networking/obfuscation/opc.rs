pub fn opc_transform(data: &[u8]) -> Vec<u8> {
    data.iter().rev().cloned().collect()
}

pub fn opc_reverse(data: &[u8]) -> Vec<u8> {
    data.iter().rev().cloned().collect()
}
