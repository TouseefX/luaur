use luaur_analysis::records::symbol::Symbol;
use luaur_ast::records::ast_name::AstName;
use std::ffi::CString;

pub fn mk_symbol(s: &str) -> Symbol {
    let name = CString::new(s).unwrap().into_raw();
    let name = AstName::ast_name_c_char(name);
    Symbol::from_global(name)
}
