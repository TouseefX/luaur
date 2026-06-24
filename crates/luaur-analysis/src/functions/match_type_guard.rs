use crate::records::type_guard::TypeGuard;
use alloc::string::String;
use core::ffi::CStr;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_binary::AstExprBinary;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;

pub fn match_type_guard(op: i32, left: *mut AstExpr, right: *mut AstExpr) -> Option<TypeGuard> {
    if op != AstExprBinary::CompareEq as i32 && op != AstExprBinary::CompareNe as i32 {
        return None;
    }

    let mut left = left;
    let mut right = right;

    if unsafe { !right.is_null() && !ast_node_as::<AstExprCall>(right as *mut AstNode).is_null() } {
        core::mem::swap(&mut left, &mut right);
    }

    let call = unsafe { ast_node_as::<AstExprCall>(left as *mut AstNode) };
    let string = unsafe { ast_node_as::<AstExprConstantString>(right as *mut AstNode) };

    if call.is_null() || string.is_null() {
        return None;
    }

    let call = unsafe { &*call };
    let string = unsafe { &*string };

    let callee = unsafe { ast_node_as::<AstExprGlobal>(call.func as *mut AstNode) };
    if callee.is_null() {
        return None;
    }

    let callee_name = unsafe { (*callee).name.value };
    if callee_name.is_null() {
        return None;
    }

    let name_bytes = unsafe { CStr::from_ptr(callee_name).to_bytes() };
    let is_typeof = if name_bytes == b"typeof" {
        true
    } else if name_bytes == b"type" {
        false
    } else {
        return None;
    };

    if call.args.len() != 1 {
        return None;
    }

    let type_str = {
        let slice = string.value.as_slice();
        let bytes =
            unsafe { core::slice::from_raw_parts(slice.as_ptr() as *const u8, slice.len()) };
        String::from_utf8_lossy(bytes).into_owned()
    };

    Some(TypeGuard {
        is_typeof,
        target: call.args.as_slice()[0],
        r#type: type_str,
    })
}
