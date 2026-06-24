use alloc::string::String;
use luaur_ast::records::location::Location;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DuplicateTypeDefinition {
    pub(crate) name: String,
    pub(crate) previousLocation: Option<Location>,
}

impl DuplicateTypeDefinition {
    pub const fn new(name: String, previous_location: Option<Location>) -> Self {
        Self {
            name,
            previousLocation: previous_location,
        }
    }
}

#[allow(non_snake_case)]
impl DuplicateTypeDefinition {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn previousLocation(&self) -> Option<Location> {
        self.previousLocation
    }
}
