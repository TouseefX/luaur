use crate::records::symbol::Symbol;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_stat_local::AstStatLocal;

pub fn find_rhs_expr_symbol_ast_stat_local(sym: Symbol, source: *mut AstStatLocal) -> *mut AstExpr {
    unsafe {
        let source = &*source;
        if sym.local.is_null() {
            return core::ptr::null_mut();
        }

        for i in 0..source.vars.size {
            let var = *source.vars.data.add(i);
            if var == sym.local && i < source.values.size {
                return *source.values.data.add(i);
            }
        }
        core::ptr::null_mut()
    }
}
