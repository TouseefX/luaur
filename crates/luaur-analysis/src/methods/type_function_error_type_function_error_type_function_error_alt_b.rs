use crate::records::type_function_error::TypeFunctionError;
use crate::type_aliases::module_name_type_fwd::ModuleName;
use crate::type_aliases::type_function_error_data::TypeFunctionErrorData;
use luaur_ast::records::location::Location;

impl TypeFunctionError {
    pub fn type_function_error_location_module_name_type_function_error_data(
        location: Location,
        module_name: ModuleName,
        data: TypeFunctionErrorData,
    ) -> Self {
        Self {
            location,
            module_name,
            data,
        }
    }
}
