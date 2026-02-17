use aes_gcm::{aead::{Aead, KeyInit}, Aes256Gcm, Nonce};
use rand::Rng;
use serde_json;

use crate::core::quantizer::QuantizedSignal;

pub struct Transmitter {
    node_id: String
}

impl Transmitter {
    pub fn new(node_id: &str) -> Self {
        Self { node_id: node_id.to_string() }
    }

    pub async fn encrypt_and_send(&self, signal: &QuantizedSignal) -> Vec<u8> {
        let key = Aes256Gcm::generate_key(&mut rand::thread_rng());
        let nonce = Nonce::from_slice(&[0; 12]);
        let cipher = Aes256Gcm::new(&key);

        let payload = serde_json::to_vec(signal).unwrap();
        let ciphertext = cipher.encrypt(nonce, payload.as_ref()).unwrap();
        println!("📡 Transmitting secure packet from {:?}", self.node_id);
        ciphertext
    }
}
