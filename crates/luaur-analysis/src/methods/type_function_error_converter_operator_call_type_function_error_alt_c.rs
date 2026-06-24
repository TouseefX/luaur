use crate::records::runtime_error::RuntimeError;
use crate::records::type_function_error_converter::TypeFunctionErrorConverter;
use alloc::string::String;

impl TypeFunctionErrorConverter {
    pub fn operator_call_2(&self, e: &RuntimeError) -> String {
        String::from(e.message())
    }
}
