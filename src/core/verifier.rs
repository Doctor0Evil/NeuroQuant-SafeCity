use crate::core::signal::{Layer, Meta, Packet};

pub struct Verifier;

impl Verifier {
    pub fn new() -> Self {
        Self
    }

    pub fn verify_meta(&self, meta: &Meta) -> bool {
        !meta.source.is_empty()
            && !meta.destination.is_empty()
            && meta.priority <= 10
            && meta.authorized
    }

    pub fn verify_transition<T>(&self, packet: &Packet<T>, target: &Layer) -> bool {
        match (&packet.meta.layer, target) {
            (Layer::Intent, Layer::NeuralEncoding) => true,
            (Layer::NeuralEncoding, Layer::Transmission) => true,
            (Layer::Transmission, Layer::NetworkRouting) => true,
            (Layer::NetworkRouting, Layer::Actuation) => true,
            _ => false,
        }
    }
}
