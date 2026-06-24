//! C++ `LintUninitializedLocal::visitAssign` (`Analysis/src/Linter.cpp:2184`).
//!
//! The `LintUninitializedLocal` record carries a placeholder no-op `visit_assign`
//! method, so the faithful logic lives here as a free function over
//! `&mut LintUninitializedLocal` and is invoked from the assign/function visitors.

use crate::records::lint_uninitialized_local::LintUninitializedLocal;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_node::AstNode;

pub fn lint_uninitialized_local_visit_assign(pass: &mut LintUninitializedLocal, var: *mut AstExpr) {
    unsafe {
        let lv = luaur_ast::rtti::ast_node_as::<AstExprLocal>(var as *mut AstNode);
        if !lv.is_null() {
            let l = pass.locals.get_or_insert((*lv).local);
            l.assigned = true;
        } else {
            luaur_ast::visit::ast_expr_visit(var, pass);
        }
    }
}
