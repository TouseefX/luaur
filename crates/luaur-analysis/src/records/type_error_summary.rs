use crate::type_aliases::module_name_type::ModuleName;
use luaur_ast::records::location::Location;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeErrorSummary {
    pub location: Location,
    pub module_name: ModuleName,
    pub code: i32,
}
