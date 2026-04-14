pub fn obfuscate(data: &[u8]) -> Vec<u8> {
    // simple XOR placeholder
    data.iter().map(|b| b ^ 0xAA).collect()
}

pub fn deobfuscate(data: &[u8]) -> Vec<u8> {
    data.iter().map(|b| b ^ 0xAA).collect()
}
