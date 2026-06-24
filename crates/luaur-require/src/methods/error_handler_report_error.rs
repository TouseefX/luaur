use crate::records::error_handler::ErrorHandler;

impl dyn ErrorHandler {
    #[allow(unused_variables)]
    pub fn report_error(&mut self, message: alloc::string::String) {}
}
