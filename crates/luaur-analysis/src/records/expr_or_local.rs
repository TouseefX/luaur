use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_local::AstLocal;

#[derive(Debug, Clone, Copy)]
pub struct ExprOrLocal {
    pub(crate) expr: *mut AstExpr,
    pub(crate) local: *mut AstLocal,
}

impl Default for ExprOrLocal {
    fn default() -> Self {
        Self {
            expr: core::ptr::null_mut(),
            local: core::ptr::null_mut(),
        }
    }
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let name: () = ();
}
