use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct NeuralIntent {
    pub user_id: String,
    pub command: String,
    pub frequency_band: f32,
    pub time_slot: u64,
}

pub struct NeuralEncoder {
    user_id: String,
}

impl NeuralEncoder {
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
        }
    }

    pub fn encode_intent(&self, raw_intent: &str) -> NeuralIntent {
        let mut rng = rand::thread_rng();
        let freq_band = rng.gen_range(0.5..50.0);
        let slot = rng.gen_range(1000..9999);

        NeuralIntent {
            user_id: self.user_id.clone(),
            command: raw_intent.to_string(),
            frequency_band: freq_band,
            time_slot: slot,
        }
    }
}
