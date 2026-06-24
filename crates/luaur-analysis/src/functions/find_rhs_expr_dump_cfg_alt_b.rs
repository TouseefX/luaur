use crate::records::symbol::Symbol;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_stat_assign::AstStatAssign;

pub fn find_rhs_expr_symbol_ast_stat_assign(
    sym: Symbol,
    source: *mut AstStatAssign,
) -> *mut AstExpr {
    unsafe {
        let source = &*source;
        for i in 0..source.vars.size {
            if i >= source.values.size {
                continue;
            }

            let var = *source.vars.data.add(i);
            if !sym.local.is_null() {
                let expr_local = luaur_ast::rtti::ast_node_as::<AstExprLocal>(
                    var as *mut luaur_ast::records::ast_node::AstNode,
                );
                if !expr_local.is_null() && (*expr_local).local == sym.local {
                    return *source.values.data.add(i);
                }
            } else if !sym.global.value.is_null() {
                let expr_global = luaur_ast::rtti::ast_node_as::<AstExprGlobal>(
                    var as *mut luaur_ast::records::ast_node::AstNode,
                );
                if !expr_global.is_null() && (*expr_global).name.value == sym.global.value {
                    return *source.values.data.add(i);
                }
            }
        }
        core::ptr::null_mut()
    }
}
