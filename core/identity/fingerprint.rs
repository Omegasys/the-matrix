use crate::identity::keys::KeyPair;
use sha2::{Sha256, Digest};

pub struct Fingerprint {
    pub hash: Vec<u8>,
}

impl Fingerprint {
    pub fn from_key(keypair: &KeyPair) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(&keypair.public);
        let result = hasher.finalize();

        Self { hash: result.to_vec() }
    }
}
