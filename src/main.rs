mod core;
mod neural;
mod safety;
mod actuation;
mod net;

use crate::actuation::control::ActuationController;
use crate::core::quantizer::Quantizer;
use crate::core::router::Router;
use crate::core::signal::{Layer, Packet};
use crate::core::verifier::Verifier;
use crate::neural::encoder::NeuralEncoder;
use crate::neural::isolator::NeuralIsolator;
use crate::net::transmission::Transmitter;
use crate::safety::monitor::SafetyMonitor;

#[tokio::main]
async fn main() {
    let intent_str = "Open floodgate 3";

    let encoder = NeuralEncoder::new("user_alpha");
    let isolator = NeuralIsolator::new();
    let quantizer = Quantizer::new();
    let router = Router::new();
    let verifier = Verifier::new();
    let transmitter = Transmitter::new("node_floodgate_3");
    let mut monitor = SafetyMonitor::new();
    let controller = ActuationController::new();

    let isolation_profile = isolator.allocate_profile("user_alpha");

    let neural_intent = encoder.encode_intent(intent_str);
    println!(
        "🧠 Intent encoded for {} on band {:.2}-{:.2} Hz, slot {}-{}",
        neural_intent.user_id,
        isolation_profile.frequency_band_start,
        isolation_profile.frequency_band_end,
        isolation_profile.time_slot_start,
        isolation_profile.time_slot_end
    );

    let meta = Router::build_meta(
        "user_alpha",
        "node_floodgate_3",
        Layer::Intent,
        5,
        true,
    );

    let packet_intent = Packet {
        meta,
        payload: neural_intent.command.clone(),
    };

    if !verifier.verify_meta(&packet_intent.meta) {
        println!("❌ Meta verification failed at Intent layer.");
        return;
    }

    let next_layer = router.route(&packet_intent).expect("routing failed");
    let packet_next = router.update_layer(packet_intent, next_layer);

    let quantized = quantizer.quantize_signal(&packet_next.payload);
    monitor.analyze_quantized(&quantized);

    if !monitor.is_safe_quantized(&quantized) {
        println!("🛑 Quantized signal flagged as unsafe.");
        return;
    }

    let secure_packet = transmitter.encrypt_and_send(&quantized).await;
    monitor.analyze_packet_bytes(&secure_packet);

    if !monitor.is_safe_bytes(&secure_packet) {
        println!("⚠️ Unsafe or anomalous packet detected. Actuation rejected.");
        return;
    }

    controller.process_secure_packet(&secure_packet).await;
}
