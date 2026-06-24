use crate::records::ast_array_predicate::AstArrayPredicate;
use core::ffi::c_char;
use luaur_ast::records::ast_array::AstArray;
use luaur_common::functions::hash_range::hashRange;

impl AstArrayPredicate {
    #[inline]
    pub fn operator_call_2(&self, value: *const AstArray<c_char>) -> usize {
        unsafe {
            let value_ref = &*value;
            hashRange(value_ref.begin() as *const c_char, value_ref.len())
        }
    }
}
