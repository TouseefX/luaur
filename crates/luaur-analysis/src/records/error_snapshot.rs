use luaur_ast::records::location::Location;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ErrorSnapshot {
    pub message: alloc::string::String,
    pub location: Location,
}
