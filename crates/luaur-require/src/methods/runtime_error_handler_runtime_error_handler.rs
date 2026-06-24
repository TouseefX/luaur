use crate::records::runtime_error_handler::RuntimeErrorHandler;

impl RuntimeErrorHandler {
    pub fn new(required_path: alloc::string::String) -> Self {
        let mut error_prefix = alloc::string::String::from("error requiring module \"");
        error_prefix.push_str(&required_path);
        error_prefix.push_str("\": ");

        Self {
            error_prefix,
            error_message: alloc::string::String::new(),
        }
    }
}

#[allow(non_snake_case)]
pub fn runtime_error_handler_runtime_error_handler(
    required_path: alloc::string::String,
) -> RuntimeErrorHandler {
    RuntimeErrorHandler::new(required_path)
}
