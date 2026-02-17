use sha2::{Sha256, Digest};

pub struct SafetyMonitor;

impl SafetyMonitor {
    pub fn new() -> Self { Self }

    pub fn analyze_packet(&self, packet: &[u8]) {
        let hash = Sha256::digest(packet);
        println!("🧩 Packet integrity hash: {:x}", hash);
    }

    pub fn is_safe(&self, packet: &[u8]) -> bool {
        packet.len() > 10 && packet.len() < 2048 // simple anomaly heuristic
    }
}
