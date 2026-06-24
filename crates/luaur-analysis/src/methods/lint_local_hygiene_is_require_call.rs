use crate::records::lint_local_hygiene::LintLocalHygiene;
use core::ffi::c_void;
use core::ffi::CStr;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;

impl LintLocalHygiene {
    pub fn is_require_call(&mut self, expr: *mut AstExpr) -> bool {
        let call = unsafe { ast_node_as::<AstExprCall>(expr as *mut AstNode) };
        if call.is_null() {
            return false;
        }

        let call_ref = unsafe { &*call };
        let glob = unsafe { ast_node_as::<AstExprGlobal>(call_ref.func as *mut AstNode) };
        if glob.is_null() {
            return false;
        }

        let glob_ref = unsafe { &*glob };
        let name_ptr = glob_ref.name.value;
        let is_require = unsafe {
            let c_str = CStr::from_ptr(name_ptr);
            c_str.to_bytes() == b"require"
        };

        is_require
    }
}
