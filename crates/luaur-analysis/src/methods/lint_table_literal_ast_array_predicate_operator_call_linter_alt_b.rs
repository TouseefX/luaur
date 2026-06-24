use crate::records::ast_array_predicate::AstArrayPredicate;
use core::ffi::c_char;
use luaur_ast::records::ast_array::AstArray;

impl AstArrayPredicate {
    #[inline]
    pub fn operator_call(
        &self,
        lhs: *const AstArray<c_char>,
        rhs: *const AstArray<c_char>,
    ) -> bool {
        if !lhs.is_null() && !rhs.is_null() {
            unsafe {
                let lhs_ref = &*lhs;
                let rhs_ref = &*rhs;

                if lhs_ref.len() != rhs_ref.len() {
                    return false;
                }

                if lhs_ref.is_empty() {
                    return true;
                }

                libc::memcmp(
                    lhs_ref.begin() as *const core::ffi::c_void,
                    rhs_ref.begin() as *const core::ffi::c_void,
                    lhs_ref.len(),
                ) == 0
            }
        } else {
            lhs == rhs
        }
    }
}

mod libc {
    use core::ffi::{c_int, c_void};
    extern "C" {
        pub fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    }
}
