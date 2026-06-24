use crate::records::runtime_luau_config_timer::RuntimeLuauConfigTimer;

impl RuntimeLuauConfigTimer {
    pub fn start(&mut self, timeout_ms: i32) {
        self.start_time = std::time::Instant::now();
        if timeout_ms < 0 {
            self.timeout_duration = None;
        } else {
            self.timeout_duration = Some(std::time::Duration::from_millis(timeout_ms as u64));
        }
    }
}
