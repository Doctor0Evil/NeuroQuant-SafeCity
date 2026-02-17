use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuantizedSignal {
    pub timestamp: i64,
    pub data: Vec<u8>,
    pub entropy: f64,
    pub checksum: String,
}

pub struct Quantizer;

impl Quantizer {
    pub fn new() -> Self {
        Self
    }

    pub fn quantize_signal(&self, input: &str) -> QuantizedSignal {
        let timestamp = chrono::Utc::now().timestamp();
        let mut rng = rand::thread_rng();
        let entropy: f64 = rng.gen();
        let data = input.as_bytes().to_vec();
        let checksum = format!("{:x}", Sha256::digest(&data));

        QuantizedSignal {
            timestamp,
            data,
            entropy,
            checksum,
        }
    }
}
