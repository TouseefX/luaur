use crate::type_aliases::module_name_type::ModuleName;
use crate::type_aliases::type_error_data::TypeErrorData;
use luaur_ast::records::location::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeError {
    pub location: Location,
    pub module_name: ModuleName,
    pub data: TypeErrorData,
}
