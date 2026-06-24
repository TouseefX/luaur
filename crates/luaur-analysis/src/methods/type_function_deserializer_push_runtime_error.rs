use crate::records::runtime_error::RuntimeError;
use crate::records::type_function_deserializer::TypeFunctionDeserializer;
use crate::records::type_function_error::TypeFunctionError;
use crate::type_aliases::type_function_error_data::TypeFunctionErrorData;
use alloc::string::String;
use luaur_ast::records::location::Location;

impl TypeFunctionDeserializer {
    pub fn push_runtime_error(&mut self, message: String) {
        if self.state.is_null() {
            return;
        }

        unsafe {
            if luaur_common::FFlag::LuauTypeFunctionStructuredErrors.get() {
                (*self.state).errors.push(TypeFunctionError {
                    location: Location::default(),
                    module_name: String::new(),
                    data: TypeFunctionErrorData::V2(RuntimeError::new(message)),
                });
            } else {
                (*self.state).errors_deprecated.push(message);
            }
        }
    }
}
