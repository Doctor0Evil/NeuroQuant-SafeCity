use sha2::{Digest, Sha256};

use crate::core::quantizer::QuantizedSignal;

pub struct SafetyMonitor {
    catastrophic_flag: bool,
}

impl SafetyMonitor {
    pub fn new() -> Self {
        Self {
            catastrophic_flag: false,
        }
    }

    pub fn analyze_packet_bytes(&self, packet: &[u8]) {
        let hash = Sha256::digest(packet);
        println!("🧩 Packet integrity hash: {:x}", hash);
    }

    pub fn analyze_quantized(&mut self, signal: &QuantizedSignal) {
        let expected = format!("{:x}", Sha256::digest(&signal.data));
        if expected != signal.checksum {
            println!("⚠️ Checksum mismatch detected.");
            self.catastrophic_flag = true;
        }

        if signal.entropy < 0.01 || signal.entropy > 0.99 {
            println!("⚠️ Entropy anomaly detected.");
            self.catastrophic_flag = true;
        }
    }

    pub fn is_safe_bytes(&self, packet: &[u8]) -> bool {
        packet.len() > 10 && packet.len() < 4096 && !self.catastrophic_flag
    }

    pub fn is_safe_quantized(&self, _signal: &QuantizedSignal) -> bool {
        !self.catastrophic_flag
    }

    pub fn catastrophic(&self) -> bool {
        self.catastrophic_flag
    }
}
