use x25519_dalek::{EphemeralSecret, PublicKey};
use rand_core::OsRng;

pub struct KeyPair {
    pub private: EphemeralSecret,
    pub public: PublicKey,
}

impl KeyPair {
    pub fn generate() -> Self {
        let private = EphemeralSecret::new(OsRng);
        let public = PublicKey::from(&private);

        Self { private, public }
    }

    pub fn shared_secret(
        private: EphemeralSecret,
        peer_public: PublicKey,
    ) -> [u8; 32] {
        private.diffie_hellman(&peer_public).to_bytes()
    }
}
