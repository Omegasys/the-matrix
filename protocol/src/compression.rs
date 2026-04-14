use zstd::stream::{encode_all, decode_all};
use std::io::Cursor;

pub struct Compression;

impl Compression {
    pub fn compress(data: &[u8]) -> Result<Vec<u8>, String> {
        encode_all(Cursor::new(data), 3)
            .map_err(|e| e.to_string())
    }

    pub fn decompress(data: &[u8]) -> Result<Vec<u8>, String> {
        decode_all(Cursor::new(data))
            .map_err(|e| e.to_string())
    }
}
