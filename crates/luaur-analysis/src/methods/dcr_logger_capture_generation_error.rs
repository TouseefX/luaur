use crate::functions::to_string_error::to_string_type_error;
use crate::records::dcr_logger::DcrLogger;
use crate::records::error_snapshot::ErrorSnapshot;
use crate::records::type_error::TypeError;
use alloc::string::String;

impl DcrLogger {
    pub fn capture_generation_error(&mut self, error: &TypeError) {
        let stringified_error: String = to_string_type_error(error);
        self.generation_log.errors.push(ErrorSnapshot {
            message: stringified_error,
            location: error.location,
        });
    }
}
