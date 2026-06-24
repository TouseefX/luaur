use crate::type_aliases::module_name_file_resolver::ModuleName;
use luaur_ast::records::location::Location;

#[derive(Debug, Clone)]
pub struct RequireCycle {
    pub location: Location,
    pub path: alloc::vec::Vec<ModuleName>,
}
