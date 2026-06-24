use crate::records::runtime_luau_config_timer::RuntimeLuauConfigTimer;

impl RuntimeLuauConfigTimer {
    pub fn is_finished(&self) -> bool {
        if let Some(timeout_duration) = self.timeout_duration {
            return self.start_time.elapsed() >= timeout_duration;
        }
        false
    }
}
