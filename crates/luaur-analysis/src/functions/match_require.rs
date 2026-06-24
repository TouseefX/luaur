use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_node::AstNode;

use luaur_ast::rtti::ast_node_as;

pub fn match_require(call: &AstExprCall) -> Option<*mut AstExpr> {
    const REQUIRE: &str = "require";

    if call.args.len() != 1 {
        return None;
    }

    let func_as_global = unsafe { ast_node_as::<AstExprGlobal>(call.func as *mut AstNode) };
    if func_as_global.is_null() {
        return None;
    }

    let name_ptr = unsafe { (*func_as_global).name.value };
    if name_ptr.is_null() {
        return None;
    }

    let name_bytes = unsafe { core::ffi::CStr::from_ptr(name_ptr).to_bytes() };
    if name_bytes != REQUIRE.as_bytes() {
        return None;
    }

    Some(unsafe { *call.args.begin() })
}
