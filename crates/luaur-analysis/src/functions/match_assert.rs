use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_global::AstExprGlobal;

pub fn match_assert(call: &AstExprCall) -> bool {
    if call.args.len() < 1 {
        return false;
    }

    let func_as_global = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprGlobal>(
            call.func as *mut luaur_ast::records::ast_node::AstNode,
        )
    };

    if func_as_global.is_null() {
        return false;
    }

    unsafe {
        let name = (*func_as_global).name.value;
        if name.is_null() {
            return false;
        }

        let name_bytes = core::ffi::CStr::from_ptr(name).to_bytes();
        name_bytes == b"assert"
    }
}
