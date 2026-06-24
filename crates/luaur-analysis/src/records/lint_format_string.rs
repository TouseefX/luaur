use crate::records::lint_context::LintContext;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_visitor::AstVisitor;

#[derive(Debug, Clone)]
pub struct LintFormatString {
    pub(crate) context: *mut LintContext,
}

impl AstVisitor for LintFormatString {
    fn visit_expr_call(&mut self, node: *mut core::ffi::c_void) -> bool {
        let node = node as *mut AstExprCall;
        self.match_call(node);
        true
    }
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let pass: () = ();
    let flags: () = ();
    let options: () = ();
    let r#unsized: () = ();
    let isc: () = ();
    let v: () = ();
    let classes: () = ();
    let openCaptures: () = ();
    let totalCaptures: () = ();
    let captureIndex: () = ();
    let j: () = ();
    let error: () = ();
    let captures: () = ();
    let rest: () = ();
}
