use crate::core::signal::{Layer, Meta, Packet};

pub struct Router;

impl Router {
    pub fn new() -> Self {
        Self
    }

    pub fn route<T>(&self, packet: &Packet<T>) -> Option<Layer> {
        if !packet.meta.authorized {
            return None;
        }

        match packet.meta.layer {
            Layer::Intent => Some(Layer::NeuralEncoding),
            Layer::NeuralEncoding => Some(Layer::Transmission),
            Layer::Transmission => Some(Layer::NetworkRouting),
            Layer::NetworkRouting => Some(Layer::Actuation),
            Layer::Actuation => None,
        }
    }

    pub fn update_layer<T>(&self, mut packet: Packet<T>, next: Layer) -> Packet<T> {
        packet.meta.layer = next;
        packet
    }

    pub fn build_meta(
        source: &str,
        destination: &str,
        layer: Layer,
        priority: u8,
        authorized: bool,
    ) -> Meta {
        Meta {
            source: source.to_string(),
            destination: destination.to_string(),
            layer,
            priority,
            authorized,
        }
    }
}
