use crate::identity::keys::KeyPair;
use crate::identity::fingerprint::Fingerprint;

pub struct Identity {
    pub keypair: KeyPair,
    pub fingerprint: Fingerprint,
}

impl Identity {
    pub fn new() -> Self {
        let keypair = KeyPair::generate();
        let fingerprint = Fingerprint::from_key(&keypair);
        Self { keypair, fingerprint }
    }

    pub fn rotate(&mut self) {
        self.keypair = KeyPair::generate();
        self.fingerprint = Fingerprint::from_key(&self.keypair);
    }
}
