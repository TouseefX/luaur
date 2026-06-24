use crate::records::error_handler::ErrorHandler;

#[derive(Debug, Clone)]
pub struct RuntimeErrorHandler {
    pub(crate) error_prefix: alloc::string::String,
    pub(crate) error_message: alloc::string::String,
}
