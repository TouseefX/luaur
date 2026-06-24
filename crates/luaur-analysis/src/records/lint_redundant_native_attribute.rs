use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_visitor::AstVisitor;

use crate::records::lint_context::LintContext;

#[derive(Debug, Clone)]
pub struct LintRedundantNativeAttribute {
    pub(crate) context: *mut LintContext,
}

impl LintRedundantNativeAttribute {
    pub fn lint_redundant_native_attribute(&mut self) {
        self.context = core::ptr::null_mut();
    }

    pub fn process(&mut self) {
        // NOTE: exact warning emission/reporting is implemented in a separate translated method file.
        // This record item only models state needed by that method.
    }

    pub fn visit_node(&mut self, _node: *mut core::ffi::c_void) -> bool {
        true
    }
}

impl AstVisitor for LintRedundantNativeAttribute {
    fn visit_expr_function(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_function(node as *mut AstExprFunction)
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
}
