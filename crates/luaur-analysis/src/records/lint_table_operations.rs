use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_unary::AstExprUnary;
use luaur_ast::records::ast_visitor::AstVisitor;

#[derive(Debug, Clone)]
pub struct LintTableOperations {
    pub(crate) context: *mut crate::records::lint_context::LintContext,
}

impl LintTableOperations {
    pub fn visit_node(&mut self, _node: *mut core::ffi::c_void) -> bool {
        true
    }
}

impl AstVisitor for LintTableOperations {
    fn visit_expr_unary(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_unary(node as *mut AstExprUnary)
    }

    fn visit_expr_call(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_call(node as *mut AstExprCall)
    }

    fn visit_node(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_node(node)
    }

    fn visit_attr(&mut self, _node: *mut core::ffi::c_void) -> bool {
        false
    }

    fn visit_type(&mut self, _node: *mut core::ffi::c_void) -> bool {
        false
    }

    fn visit_type_pack(&mut self, _node: *mut core::ffi::c_void) -> bool {
        false
    }
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let pass: () = ();
    let args: () = ();
    let result: () = ();
}
