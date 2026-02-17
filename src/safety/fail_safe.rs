use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemMode {
    Normal,
    SafeMode,
    ManualOverride,
}

#[derive(Clone)]
pub struct FailSafe {
    mode: Arc<RwLock<SystemMode>>,
}

impl FailSafe {
    pub fn new() -> Self {
        Self {
            mode: Arc::new(RwLock::new(SystemMode::Normal)),
        }
    }

    pub fn set_mode(&self, new_mode: SystemMode) {
        if let Ok(mut guard) = self.mode.write() {
            *guard = new_mode;
        }
    }

    pub fn get_mode(&self) -> SystemMode {
        if let Ok(guard) = self.mode.read() {
            *guard
        } else {
            SystemMode::SafeMode
        }
    }

    pub fn allow_actuation(&self) -> bool {
        matches!(self.get_mode(), SystemMode::Normal | SystemMode::ManualOverride)
    }
}
