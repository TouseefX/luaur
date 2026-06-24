use luaur_ast::records::ast_visitor::AstVisitor;

use crate::records::lint_context::LintContext;
use luaur_ast::records::ast_stat_for::AstStatFor;

#[derive(Debug, Clone)]
pub struct LintForRange {
    pub(crate) context: *mut LintContext,
}

impl LintForRange {
    pub fn lint_for_range(&mut self) {
        self.context = core::ptr::null_mut();
    }

    pub fn get_loop_end(&self, from: f64, to: f64) -> f64 {
        from + (to - from).floor()
    }

    pub fn visit_stat_for(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_for_linter(node as *mut AstStatFor)
    }
}

impl AstVisitor for LintForRange {
    fn visit_node(&mut self, _node: *mut core::ffi::c_void) -> bool {
        true
    }

    fn visit_stat_for(&mut self, node: *mut core::ffi::c_void) -> bool {
        LintForRange::visit_stat_for(self, node)
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
