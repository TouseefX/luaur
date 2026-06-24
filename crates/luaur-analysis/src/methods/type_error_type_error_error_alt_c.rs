use crate::records::type_error::TypeError;
use crate::type_aliases::type_error_data::TypeErrorData;
use luaur_ast::records::location::Location;

impl TypeError {
    pub fn type_error_location_type_error_data(location: Location, data: TypeErrorData) -> Self {
        Self {
            location,
            module_name: alloc::string::String::new(),
            data,
        }
    }
}
