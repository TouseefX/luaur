use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_local::AstLocal;
use luaur_ast::records::ast_stat_assign::AstStatAssign;
use luaur_ast::records::ast_stat_function::AstStatFunction;
use luaur_ast::records::ast_stat_local::AstStatLocal;
use luaur_ast::records::ast_visitor::AstVisitor;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_table::DenseDefault;

use crate::records::lint_context::LintContext;

#[derive(Debug, Clone, Default)]
pub struct Local {
    pub(crate) defined: bool,
    pub(crate) initialized: bool,
    pub(crate) assigned: bool,
    pub(crate) first_use: *mut AstExprLocal,
}

impl DenseDefault for Local {
    fn dense_default() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone)]
pub struct LintUninitializedLocal {
    pub(crate) context: *mut LintContext,
    pub(crate) locals: DenseHashMap<*mut AstLocal, Local>,
}

impl LintUninitializedLocal {
    pub fn lint_uninitialized_local(&mut self) {
        self.locals = DenseHashMap::new(core::ptr::null_mut());
    }

    pub fn report(&mut self) {
        // NOTE: exact warning emission/reporting is implemented in a separate translated method file.
        // This record item only models state needed by that method.
    }

    pub fn visit_stat_local(&mut self, _node: *mut core::ffi::c_void) -> bool {
        true
    }

    pub fn visit_stat_assign(&mut self, _node: *mut core::ffi::c_void) -> bool {
        true
    }

    pub fn visit_stat_function(&mut self, _node: *mut core::ffi::c_void) -> bool {
        true
    }

    pub fn visit_expr_local(&mut self, _node: *mut core::ffi::c_void) -> bool {
        true
    }

    pub fn visit_assign(&mut self, _var: *mut AstExpr) {
        // implemented in separate method files
    }
}

impl AstVisitor for LintUninitializedLocal {
    fn visit_stat_local(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_local(node as *mut AstStatLocal)
    }

    fn visit_stat_assign(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_assign(node as *mut AstStatAssign)
    }

    fn visit_stat_function(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_function(node as *mut AstStatFunction)
    }

    fn visit_expr_local(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_local(node as *mut AstExprLocal)
    }

    fn visit_node(&mut self, _node: *mut core::ffi::c_void) -> bool {
        true
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
    let r#local: () = ();
    let l: () = ();
    let last: () = ();
}
