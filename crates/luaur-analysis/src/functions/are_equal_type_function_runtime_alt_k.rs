use crate::functions::are_equal_type_function_runtime_alt_m::are_equal_are_equal_state_type_function_type_type_function_type;
use crate::functions::are_equal_type_function_runtime_alt_p::are_equal_are_equal_state_type_function_type_pack_var_type_function_type_pack_var;
use crate::functions::seen_set_contains::seen_set_contains;
use crate::records::are_equal_state::AreEqualState;
use crate::records::type_function_function_type::TypeFunctionFunctionType;

#[allow(non_snake_case)]
pub fn are_equal_are_equal_state_type_function_function_type_type_function_function_type(
    seen: &mut AreEqualState,
    lhs: &TypeFunctionFunctionType,
    rhs: &TypeFunctionFunctionType,
) -> bool {
    if seen_set_contains(
        seen,
        lhs as *const TypeFunctionFunctionType as *const core::ffi::c_void,
        rhs as *const TypeFunctionFunctionType as *const core::ffi::c_void,
    ) {
        return true;
    }

    if lhs.generics.len() != rhs.generics.len() {
        return false;
    }

    for i in 0..lhs.generics.len() {
        let l = lhs.generics[i];
        let r = rhs.generics[i];
        if !are_equal_are_equal_state_type_function_type_type_function_type(
            seen,
            unsafe { &*l },
            unsafe { &*r },
        ) {
            return false;
        }
    }

    if lhs.generic_packs.len() != rhs.generic_packs.len() {
        return false;
    }

    for i in 0..lhs.generic_packs.len() {
        let l = lhs.generic_packs[i];
        let r = rhs.generic_packs[i];
        if !are_equal_are_equal_state_type_function_type_pack_var_type_function_type_pack_var(
            seen,
            unsafe { &*l },
            unsafe { &*r },
        ) {
            return false;
        }
    }

    if lhs.arg_types.is_null() != rhs.arg_types.is_null() {
        return false;
    }

    if !lhs.arg_types.is_null() && !rhs.arg_types.is_null() {
        if !are_equal_are_equal_state_type_function_type_pack_var_type_function_type_pack_var(
            seen,
            unsafe { &*lhs.arg_types },
            unsafe { &*rhs.arg_types },
        ) {
            return false;
        }
    }

    if lhs.ret_types.is_null() != rhs.ret_types.is_null() {
        return false;
    }

    if !lhs.ret_types.is_null() && !rhs.ret_types.is_null() {
        if !are_equal_are_equal_state_type_function_type_pack_var_type_function_type_pack_var(
            seen,
            unsafe { &*lhs.ret_types },
            unsafe { &*rhs.ret_types },
        ) {
            return false;
        }
    }

    true
}
