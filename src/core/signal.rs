use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Layer {
    Intent,
    NeuralEncoding,
    Transmission,
    NetworkRouting,
    Actuation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub source: String,
    pub destination: String,
    pub layer: Layer,
    pub priority: u8,
    pub authorized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packet<T> {
    pub meta: Meta,
    pub payload: T,
}
