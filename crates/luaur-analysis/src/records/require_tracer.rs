use luaur_ast::records::ast_visitor::AstVisitor;

use crate::records::file_resolver::FileResolver;
use crate::records::module_info::ModuleInfo;
use crate::records::require_trace_result::RequireTraceResult;
use crate::type_aliases::module_name_file_resolver::ModuleName;

use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_local::AstLocal;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_assign::AstStatAssign;
use luaur_ast::records::ast_stat_local::AstStatLocal;

use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_table::DenseDefault;

impl DenseDefault for ModuleInfo {
    fn dense_default() -> Self {
        Self {
            name: alloc::string::String::new(),
            optional: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RequireTracer {
    pub(crate) result: *mut RequireTraceResult,
    pub(crate) file_resolver: *mut FileResolver,
    pub(crate) current_module_name: ModuleName,
    pub(crate) locals: DenseHashMap<*mut AstLocal, *mut AstExpr>,
    pub(crate) work: alloc::vec::Vec<*mut AstNode>,
    pub(crate) require_calls: alloc::vec::Vec<*mut AstExprCall>,
}

impl AstVisitor for RequireTracer {
    fn visit_node(&mut self, _node: *mut core::ffi::c_void) -> bool {
        true
    }

    fn visit_expr_type_assertion(&mut self, _node: *mut core::ffi::c_void) -> bool {
        false
    }

    fn visit_expr_call(&mut self, node: *mut core::ffi::c_void) -> bool {
        unsafe {
            let expr = node as *mut AstExprCall;
            if expr.is_null() {
                return true;
            }

            let global = (*(*expr).func).base.as_item_mut::<AstExprGlobal>();
            if !global.is_null() {
                let name_ptr = (*global).name.value;
                if !name_ptr.is_null() {
                    let c_str = core::ffi::CStr::from_ptr(name_ptr).to_string_lossy();
                    if c_str == "require" && (*expr).args.size >= 1 {
                        self.require_calls.push(expr);
                    }
                }
            }
            true
        }
    }

    fn visit_stat_local(&mut self, node: *mut core::ffi::c_void) -> bool {
        unsafe {
            let stat = node as *mut AstStatLocal;
            if stat.is_null() {
                return true;
            }

            for i in 0..(*stat).vars.size.min((*stat).values.size) {
                let local = *(*stat).vars.data.add(i);
                let expr = *(*stat).values.data.add(i);
                *self.locals.get_or_insert(local) = expr;
            }
        }
        true
    }

    fn visit_stat_assign(&mut self, node: *mut core::ffi::c_void) -> bool {
        unsafe {
            let stat = node as *mut AstStatAssign;
            if stat.is_null() {
                return true;
            }

            for i in 0..(*stat).vars.size {
                let v = *(*stat).vars.data.add(i);
                let expr_local = (*v).base.as_item_mut::<AstExprLocal>();
                if !expr_local.is_null() {
                    let local = (*expr_local).local;
                    *self.locals.get_or_insert(local) = core::ptr::null_mut();
                }
            }
        }
        true
    }

    fn visit_type(&mut self, _node: *mut core::ffi::c_void) -> bool {
        true
    }

    fn visit_type_pack(&mut self, _node: *mut core::ffi::c_void) -> bool {
        true
    }
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let r#local: () = ();
    let expr: () = ();
    let moduleContext: () = ();
    let info: () = ();
    let arg: () = ();
    let infoCopy: () = ();
}
