use luaur_ast::enums::ast_table_access::AstTableAccess;
use luaur_ast::records::location::Location;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rec {
    pub access: AstTableAccess,
    pub location: Location,
}
