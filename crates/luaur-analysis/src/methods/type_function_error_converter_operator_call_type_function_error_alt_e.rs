use crate::records::type_function_error_converter::TypeFunctionErrorConverter;
use crate::records::type_function_missing::TypeFunctionMissing;
use alloc::string::String;
use luaur_common::functions::format::format;

impl TypeFunctionErrorConverter {
    pub fn operator_call_3(&self, e: &TypeFunctionMissing) -> String {
        format(format_args!(
            "Could not find '{}' type function in the global scope",
            e.functionName()
        ))
    }
}
