mod core;
mod neural;
mod safety;
mod actuation;
mod net;

use crate::core::quantizer::Quantizer;
use crate::neural::encoder::NeuralEncoder;
use crate::net::transmission::Transmitter;
use crate::safety::monitor::SafetyMonitor;
use crate::actuation::control::ActuationController;

#[tokio::main]
async fn main() {
    println!("🧠 Starting NeuroQuant-SafeCity system...");

    let intent = "Open floodgate 3";
    let neural_encoder = NeuralEncoder::new("user_alpha");
    let quantizer = Quantizer::new();
    let transmitter = Transmitter::new("node_floodgate_3");
    let monitor = SafetyMonitor::new();
    let controller = ActuationController::new();

    let encoded = neural_encoder.encode_intent(intent);
    let quantized = quantizer.quantize_signal(&encoded);
    let secure_packet = transmitter.encrypt_and_send(&quantized).await;
    monitor.analyze_packet(&secure_packet);

    if monitor.is_safe(&secure_packet) {
        controller.process_secure_packet(&secure_packet).await;
    } else {
        println!("⚠️ Unsafe or anomalous packet detected. Actuation rejected.");
    }
}
