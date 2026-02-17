use NeuroQuant_SafeCity::{
    core::quantizer::Quantizer,
    neural::encoder::NeuralEncoder,
};

#[test]
fn quantization_produces_non_empty_data() {
    let encoder = NeuralEncoder::new("test_user");
    let intent = encoder.encode_intent("Open gate");
    let quantizer = Quantizer::new();
    let signal = quantizer.quantize_signal(&intent.command);
    assert!(!signal.data.is_empty());
}

#[test]
fn checksum_matches_data() {
    use sha2::{Digest, Sha256};

    let quantizer = Quantizer::new();
    let signal = quantizer.quantize_signal("Test command");
    let expected = format!("{:x}", Sha256::digest(&signal.data));
    assert_eq!(expected, signal.checksum);
}
