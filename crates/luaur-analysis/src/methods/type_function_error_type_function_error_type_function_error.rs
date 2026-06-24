use crate::records::type_function_error::TypeFunctionError;
use crate::type_aliases::type_function_error_data::TypeFunctionErrorData;
use luaur_ast::records::location::Location;

impl TypeFunctionError {
    pub fn type_function_error() -> Self {
        Self {
            location: Location::default(),
            module_name: alloc::string::String::new(),
            data: TypeFunctionErrorData::V0(crate::records::unsupported_type::UnsupportedType {
                r#type: core::ptr::null(),
            }),
        }
    }
}
