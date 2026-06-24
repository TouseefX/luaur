use crate::records::dcr_logger::DcrLogger;
use alloc::string::String;

impl DcrLogger {
    pub fn capture_source(&mut self, source: String) {
        self.generation_log.source = source;
    }
}
