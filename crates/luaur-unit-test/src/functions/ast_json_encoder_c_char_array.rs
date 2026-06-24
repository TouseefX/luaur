use luaur_ast::records::ast_array::AstArray;

pub fn c_char_array(items: &mut [core::ffi::c_char]) -> AstArray<core::ffi::c_char> {
    AstArray {
        data: items.as_mut_ptr(),
        size: items.len(),
    }
}
