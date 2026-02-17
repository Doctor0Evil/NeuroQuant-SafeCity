use serde_json;

use crate::core::quantizer::QuantizedSignal;
use crate::net::security::CryptoContext;

pub struct Transmitter {
    node_id: String,
    crypto: CryptoContext,
}

impl Transmitter {
    pub fn new(node_id: &str) -> Self {
        Self {
            node_id: node_id.to_string(),
            crypto: CryptoContext::new(),
        }
    }

    pub async fn encrypt_and_send(&self, signal: &QuantizedSignal) -> Vec<u8> {
        let payload = serde_json::to_vec(signal).expect("serialize signal");
        let ciphertext = self.crypto.encrypt(&payload);
        println!("📡 Transmitting secure packet from {}", self.node_id);
        ciphertext
    }

    pub fn decrypt_received(&self, ciphertext: &[u8]) -> Option<QuantizedSignal> {
        let decrypted = self.crypto.decrypt(ciphertext)?;
        serde_json::from_slice::<QuantizedSignal>(&decrypted).ok()
    }
}
