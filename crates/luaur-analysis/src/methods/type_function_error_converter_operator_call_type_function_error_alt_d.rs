use crate::records::failed_to_compile::FailedToCompile;
use crate::records::type_function_error_converter::TypeFunctionErrorConverter;
use alloc::string::String;

use luaur_common::functions::format::format;

impl TypeFunctionErrorConverter {
    pub fn operator_call(&self, e: &FailedToCompile) -> String {
        format(format_args!(
            "'{}' type function failed to compile with error message: {}",
            e.functionName(),
            e.compileError()
        ))
    }
}
