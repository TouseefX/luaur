use alloc::string::String;
use core::slice;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_interp_string::AstExprInterpString;
use luaur_ast::records::ast_node::AstNode;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn get_string_contents(node: *const AstNode) -> Option<String> {
    if node.is_null() {
        return None;
    }

    let node_ref = unsafe { &*node };

    if let Some(string_node) = unsafe { node_ref.as_item::<AstExprConstantString>().as_ref() } {
        let data = string_node.value.begin();
        let size = string_node.value.len();
        if !data.is_null() {
            let slice = unsafe { slice::from_raw_parts(data as *const u8, size) };
            return Some(String::from_utf8_lossy(slice).into_owned());
        }
    } else if let Some(interp_string) =
        unsafe { node_ref.as_item::<AstExprInterpString>().as_ref() }
    {
        if interp_string.expressions.len() == 0 {
            LUAU_ASSERT!(interp_string.strings.len() == 1);
            let first_string_array = unsafe { &*interp_string.strings.begin() };
            let data = first_string_array.begin();
            let size = first_string_array.len();
            if !data.is_null() {
                let slice = unsafe { slice::from_raw_parts(data as *const u8, size) };
                return Some(String::from_utf8_lossy(slice).into_owned());
            }
        }
    }

    None
}
