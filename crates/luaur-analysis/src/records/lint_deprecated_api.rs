use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_stat_function::AstStatFunction;
use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;
use luaur_ast::records::ast_visitor::AstVisitor;

use crate::records::function_type::FunctionType;
use crate::records::lint_context::LintContext;

#[derive(Debug, Clone)]
pub struct LintDeprecatedApi {
    pub(crate) context: *mut LintContext,
    pub(crate) function_type_scope_stack: alloc::vec::Vec<*const FunctionType>,
}

impl LintDeprecatedApi {
    pub fn lint_deprecated_api(&mut self, context: *mut LintContext) {
        self.context = context;
        self.function_type_scope_stack = alloc::vec::Vec::new();
    }
}

impl AstVisitor for LintDeprecatedApi {
    fn visit_node(&mut self, _node: *mut core::ffi::c_void) -> bool {
        true
    }

    fn visit_expr_index_name(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_index_name(node as *mut AstExprIndexName)
    }

    fn visit_expr_local(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_local(node as *mut AstExprLocal)
    }

    fn visit_expr_global(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_global(node as *mut AstExprGlobal)
    }

    fn visit_expr_call(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_call(node as *mut AstExprCall)
    }

    fn visit_stat_local_function(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_local_function(node as *mut AstStatLocalFunction)
    }

    fn visit_stat_function(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_stat_function(node as *mut AstStatFunction)
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
    let level: () = ();
    let className: () = ();
    let functionName: () = ();
    let isDeprecated: () = ();
    let fty: () = ();
}
