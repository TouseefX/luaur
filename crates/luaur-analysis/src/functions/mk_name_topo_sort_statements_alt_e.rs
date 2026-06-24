use crate::functions::mk_name_topo_sort_statements_alt_g::mk_name_ast_expr;
use crate::records::identifier::Identifier;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;

pub fn mk_name_ast_expr_index_name(expr: &AstExprIndexName) -> Option<Identifier> {
    let lhs = mk_name_ast_expr(unsafe { &*expr.expr });
    if let Some(mut lhs) = lhs {
        let index_ptr = expr.index.value;
        let index_str = if index_ptr.is_null() {
            alloc::string::String::new()
        } else {
            unsafe {
                core::ffi::CStr::from_ptr(index_ptr)
                    .to_string_lossy()
                    .into_owned()
            }
        };

        let mut s = lhs.name().to_string();
        s.push('.');
        s.push_str(&index_str);

        Some(Identifier::new(s, lhs.ctx()))
    } else {
        None
    }
}
