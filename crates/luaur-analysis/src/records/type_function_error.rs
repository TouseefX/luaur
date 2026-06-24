use crate::type_aliases::module_name_type_fwd::ModuleName;
use crate::type_aliases::type_function_error_data::TypeFunctionErrorData;
use luaur_ast::records::location::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeFunctionError {
    pub(crate) location: Location,
    pub(crate) module_name: ModuleName,
    pub(crate) data: TypeFunctionErrorData,
}
