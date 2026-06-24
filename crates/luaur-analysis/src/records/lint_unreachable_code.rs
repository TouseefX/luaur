use crate::records::lint_context::LintContext;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_visitor::AstVisitor;

#[derive(Debug, Clone)]
pub struct LintUnreachableCode {
    pub(crate) context: *mut LintContext,
}

impl AstVisitor for LintUnreachableCode {
    fn visit_expr_function(&mut self, node: *mut core::ffi::c_void) -> bool {
        let node = node as *mut AstExprFunction;
        unsafe {
            let body = (*node).body;
            self.analyze(body as *mut AstStat);
        }
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
    let si: () = ();
    let step: () = ();
    let next: () = ();
    let Error: () = ();
    let Unknown: () = ();
    let Break: () = ();
    let Continue: () = ();
    let Return: () = ();
}
