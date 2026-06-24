use crate::records::symbol::Symbol;

impl Symbol {
    pub fn symbol() -> Self {
        Symbol::symbol_ast_local(core::ptr::null_mut())
    }
}
