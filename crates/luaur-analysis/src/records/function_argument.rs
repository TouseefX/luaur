use crate::type_aliases::name_type::Name;
use luaur_ast::records::location::Location;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionArgument {
    pub name: Name,
    pub location: Location,
}
