use alloc::string::String;

use crate::enums::status_require_navigator::Status;
use crate::records::navigator::Navigator;

impl Navigator {
    pub fn navigate(&mut self, mut path: String) -> Status {
        // Replace backslashes with forward slashes
        path = path.replace('\\', "/");

        if let Some(error) = self.navigate_impl(path.as_str()) {
            // Disambiguate trait method call: ErrorHandler::report_error(message: String)
            crate::records::error_handler::ErrorHandler::report_error(
                unsafe { &mut *self.error_handler },
                error,
            );
            return Status::ErrorReported;
        }

        Status::Success
    }
}
