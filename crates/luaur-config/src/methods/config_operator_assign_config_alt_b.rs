use crate::records::config::Config;

impl Config {
    // Config& Config::operator=(const Config& other)
    pub fn operator_assign_mut(&mut self, other: &Config) -> &mut Self {
        if core::ptr::eq(self, other) {
            return self;
        }

        let mut copy = other.clone();
        core::mem::swap(self, &mut copy);
        self
    }
}
