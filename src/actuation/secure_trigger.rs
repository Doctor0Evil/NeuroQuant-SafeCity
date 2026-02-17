use crate::core::quantizer::QuantizedSignal;

#[derive(Debug)]
pub struct MechanicalInterlock {
    required_pattern: u8,
}

impl MechanicalInterlock {
    pub fn new(required_pattern: u8) -> Self {
        Self { required_pattern }
    }

    pub fn check(&self, packet: &[u8]) -> bool {
        if packet.is_empty() {
            return false;
        }
        let last = packet[packet.len() - 1];
        last & 0x0F == self.required_pattern
    }

    pub fn check_quantized(&self, signal: &QuantizedSignal) -> bool {
        if signal.data.is_empty() {
            return false;
        }
        let last = signal.data[signal.data.len() - 1];
        last & 0x0F == self.required_pattern
    }
}
