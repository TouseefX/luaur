use crate::enums::polarity::Polarity;
use crate::functions::invert_polarity::invert;
use luaur_ast::enums::ast_table_access::AstTableAccess;

pub fn polarity_of_access(access: AstTableAccess, p: Polarity) -> Polarity {
    match access {
        AstTableAccess::Read => p,
        AstTableAccess::Write => invert(p),
        AstTableAccess::ReadWrite => Polarity::Mixed,
        _ => Polarity::Unknown,
    }
}
