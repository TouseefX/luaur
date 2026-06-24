use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_constant_bool::AstExprConstantBool;
use luaur_ast::records::ast_expr_global::AstExprGlobal;

#[allow(non_snake_case)]
pub fn does_call_error(call: &AstExprCall) -> bool {
    let global = unsafe {
        luaur_ast::rtti::ast_node_as::<AstExprGlobal>(
            call.func as *mut luaur_ast::records::ast_node::AstNode,
        )
    };

    if global.is_null() {
        return false;
    }

    unsafe {
        let name = (*global).name.value;
        if name.is_null() {
            return false;
        }

        let name_bytes = core::ffi::CStr::from_ptr(name).to_bytes();
        if name_bytes == b"error" {
            return true;
        }

        if name_bytes == b"assert" {
            // assert() will error because it is missing the first argument
            let first_arg = match call.args.iter().next() {
                Some(arg) => *arg,
                None => return true,
            };

            if first_arg.is_null() {
                return false;
            }

            let expr = luaur_ast::rtti::ast_node_as::<AstExprConstantBool>(
                first_arg as *mut luaur_ast::records::ast_node::AstNode,
            );

            if expr.is_null() {
                return false;
            }

            if !(*expr).value {
                return true;
            }
        }
    }

    false
}
