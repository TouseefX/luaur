use crate::records::sym_def::SymDef;
use crate::records::symbol::Symbol;

impl SymDef {
    pub fn sym_def_sym_def(sym: Symbol, version: usize) -> Self {
        Self { sym, version }
    }
}
