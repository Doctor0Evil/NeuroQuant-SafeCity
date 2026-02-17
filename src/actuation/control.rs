// src/actuation/control.rs
use tokio::time::{sleep, Duration};

use crate::actuation::secure_trigger::MechanicalInterlock;
use crate::core::quantizer::QuantizedSignal;
use crate::safety::fail_safe::FailSafe;

pub struct ActuationController {
    interlock: MechanicalInterlock,
    fail_safe: FailSafe,
}

impl ActuationController {
    pub fn new() -> Self {
        Self {
            interlock: MechanicalInterlock::new(0x0A),
            fail_safe: FailSafe::new(),
        }
    }

    pub fn fail_safe(&self) -> FailSafe {
        self.fail_safe.clone()
    }

    pub async fn process_secure_packet(&self, packet: &[u8]) {
        println!("🔐 Verifying actuation packet bytes...");
        sleep(Duration::from_millis(500)).await;

        if !self.fail_safe.allow_actuation() {
            println!("🛑 Actuation blocked: system not in Normal or ManualOverride.");
            return;
        }

        if self.interlock.check(packet) {
            println!("✅ Floodgate actuation verified and executed safely (bytes path).");
        } else {
            println!("❌ Packet verification failed: mechanical lock maintained (bytes path).");
        }
    }

    pub async fn process_quantized(&self, signal: &QuantizedSignal) {
        println!("🔐 Verifying quantized actuation...");
        sleep(Duration::from_millis(500)).await;

        if !self.fail_safe.allow_actuation() {
            println!("🛑 Actuation blocked: system not in Normal or ManualOverride.");
            return;
        }

        if self.interlock.check_quantized(signal) {
            println!("✅ Floodgate actuation verified and executed safely (quantized path).");
        } else {
            println!("❌ Quantized verification failed: mechanical lock maintained.");
        }
    }
}
