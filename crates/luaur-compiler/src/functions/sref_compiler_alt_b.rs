use core::ffi::c_char;
use luaur_ast::records::ast_array::AstArray;
use luaur_bytecode::records::string_ref::StringRef;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn sref_ast_array_c_char_mut(data: AstArray<c_char>) -> StringRef {
    LUAU_ASSERT!(!data.begin().is_null());
    StringRef::new(data.begin() as *const core::ffi::c_char, data.len())
}
