//! Faithful port of `TypeChecker2::allowsNoReturnValues` (TypeChecker2.cpp:329-338).
//!
//! `allowsNoReturnValues` is a `static` member function. Its body was already
//! laid into the codebase as the free function
//! `functions::allows_no_return_values` (with a `&self` method wrapper on
//! `TypeChecker2`); this skeleton node is the static-method translation slot and
//! delegates to that canonical implementation rather than re-defining it (which
//! would collide with the existing `TypeChecker2::allows_no_return_values`).
use crate::type_aliases::type_pack_id::TypePackId;

pub fn type_checker_2_allows_no_return_values(tp: TypePackId) -> bool {
    crate::functions::allows_no_return_values::allows_no_return_values(tp)
}
