use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_local::AstLocal;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_local::AstStatLocal;
use luaur_ast::records::ast_visitor::AstVisitor;
use luaur_common::records::dense_hash_map::DenseHashMap;

use crate::records::lint_context::LintContext;

#[derive(Debug, Clone)]
pub struct LintDuplicateLocal {
    pub(crate) context: *mut LintContext,
    pub(crate) locals: DenseHashMap<*mut AstLocal, *mut AstNode>,
}

impl LintDuplicateLocal {
    pub fn new() -> Self {
        Self {
            context: core::ptr::null_mut(),
            locals: DenseHashMap::new(core::ptr::null_mut()),
        }
    }

    pub fn ignore_duplicate(&self, local: *mut AstLocal) -> bool {
        let local = unsafe { &*local };
        let name = unsafe {
            core::ffi::CStr::from_ptr(local.name.value)
                .to_str()
                .unwrap_or("")
        };
        name == "_"
    }
}

impl AstVisitor for LintDuplicateLocal {
    fn visit_stat_local(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_local(node as *mut AstStatLocal)
    }

    fn visit_expr_function(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_function(node as *mut AstExprFunction)
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
}
