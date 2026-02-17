use rand::Rng;

#[derive(Debug, Clone)]
pub struct IsolationProfile {
    pub frequency_band_start: f32,
    pub frequency_band_end: f32,
    pub time_slot_start: u64,
    pub time_slot_end: u64,
    pub code_id: u32,
}

pub struct NeuralIsolator;

impl NeuralIsolator {
    pub fn new() -> Self {
        Self
    }

    pub fn allocate_profile(&self, user_id: &str) -> IsolationProfile {
        let mut rng = rand::thread_rng();
        let base_freq = rng.gen_range(5.0..40.0);
        let width = rng.gen_range(1.0..5.0);
        let start_slot = rng.gen_range(1_000..9_000);
        let end_slot = start_slot + rng.gen_range(10..100);
        let code_id = Self::hash_user(user_id);

        IsolationProfile {
            frequency_band_start: base_freq,
            frequency_band_end: base_freq + width,
            time_slot_start: start_slot,
            time_slot_end: end_slot,
            code_id,
        }
    }

    fn hash_user(user_id: &str) -> u32 {
        let mut acc: u32 = 0;
        for b in user_id.as_bytes() {
            acc = acc.wrapping_mul(16777619) ^ (*b as u32);
        }
        acc
    }
}
