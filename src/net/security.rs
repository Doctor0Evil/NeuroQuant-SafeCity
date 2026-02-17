use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::RngCore;

#[derive(Clone)]
pub struct SessionKey {
    pub key_bytes: [u8; 32],
}

impl SessionKey {
    pub fn new_random() -> Self {
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        Self { key_bytes: bytes }
    }

    pub fn cipher(&self) -> Aes256Gcm {
        let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&self.key_bytes);
        Aes256Gcm::new(key)
    }
}

pub struct CryptoContext {
    pub session: SessionKey,
}

impl CryptoContext {
    pub fn new() -> Self {
        Self {
            session: SessionKey::new_random(),
        }
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
        let cipher = self.session.cipher();
        let nonce = Nonce::from_slice(&[0u8; 12]);
        cipher.encrypt(nonce, plaintext).expect("encryption failure")
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Option<Vec<u8>> {
        let cipher = self.session.cipher();
        let nonce = Nonce::from_slice(&[0u8; 12]);
        cipher.decrypt(nonce, ciphertext).ok()
    }
}
