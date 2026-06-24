use crate::records::runtime_error_handler::RuntimeErrorHandler;

impl RuntimeErrorHandler {
    pub fn get_reported_error(&self) -> &alloc::string::String {
        &self.error_message
    }
}
