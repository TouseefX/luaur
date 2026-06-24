use crate::records::type_function_error::TypeFunctionError;
use crate::type_aliases::type_function_error_data::TypeFunctionErrorData;
use luaur_ast::records::location::Location;

impl TypeFunctionError {
    pub fn type_function_error_location_type_function_error_data(
        location: Location,
        data: TypeFunctionErrorData,
    ) -> Self {
        Self {
            location,
            module_name: alloc::string::String::new(),
            data,
        }
    }
}
