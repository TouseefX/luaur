use crate::records::symbol::Symbol;
use luaur_ast::records::ast_local::AstLocal;

impl Symbol {
    pub fn symbol_ast_local(local: *mut AstLocal) -> Self {
        Symbol {
            local,
            global: luaur_ast::records::ast_name::AstName::default(),
        }
    }
}
