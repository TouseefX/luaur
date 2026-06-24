use crate::type_aliases::module_name_type::ModuleName;
use luaur_ast::records::location::Location;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionDefinition {
    pub(crate) definition_module_name: Option<ModuleName>,
    pub(crate) definition_location: Location,
    pub(crate) vararg_location: Option<Location>,
    pub(crate) original_name_location: Location,
}

#[allow(non_snake_case)]
impl FunctionDefinition {
    pub fn definitionModuleName(&self) -> Option<&ModuleName> {
        self.definition_module_name.as_ref()
    }

    pub fn definitionLocation(&self) -> Location {
        self.definition_location
    }

    pub fn varargLocation(&self) -> Option<Location> {
        self.vararg_location
    }

    pub fn originalNameLocation(&self) -> Location {
        self.original_name_location
    }
}
