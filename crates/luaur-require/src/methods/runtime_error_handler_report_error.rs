use crate::records::error_handler::ErrorHandler;
use crate::records::runtime_error_handler::RuntimeErrorHandler;

impl RuntimeErrorHandler {
    pub fn report_error(&mut self, message: alloc::string::String) {
        self.error_message = self.error_prefix.clone() + &message;
    }
}

impl ErrorHandler for RuntimeErrorHandler {
    fn report_error(&mut self, message: alloc::string::String) {
        RuntimeErrorHandler::report_error(self, message);
    }
}
