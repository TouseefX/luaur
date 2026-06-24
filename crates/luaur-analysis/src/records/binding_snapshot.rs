use luaur_ast::records::location::Location;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct BindingSnapshot {
    pub type_id: alloc::string::String,
    pub type_string: alloc::string::String,
    pub location: Location,
}
