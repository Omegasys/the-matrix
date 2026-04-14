use rand::Rng;

pub struct KeyPair {
    pub public: Vec<u8>,
    pub private: Vec<u8>,
}

impl KeyPair {
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let public: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        let private: Vec<u8> = (0..64).map(|_| rng.gen()).collect();

        Self { public, private }
    }
}
