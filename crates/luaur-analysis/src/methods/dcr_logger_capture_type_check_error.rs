use crate::records::dcr_logger::DcrLogger;
use crate::records::error_snapshot::ErrorSnapshot;
use crate::records::type_error::TypeError;

use crate::functions::to_string_error::to_string_type_error;

impl DcrLogger {
    pub fn capture_type_check_error(&mut self, error: &TypeError) {
        let stringified_error = to_string_type_error(error);
        let snapshot = ErrorSnapshot {
            message: stringified_error,
            location: error.location,
        };

        self.check_log.errors.push(snapshot);
    }
}
