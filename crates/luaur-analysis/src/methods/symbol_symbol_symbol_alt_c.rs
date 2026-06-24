use crate::records::symbol::Symbol;
use luaur_ast::records::ast_name::AstName;

impl Symbol {
    pub fn symbol_ast_name(global: AstName) -> Self {
        Symbol {
            local: core::ptr::null_mut(),
            global,
        }
    }
}
