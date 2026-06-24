use crate::functions::are_equal_type_function_runtime_alt_m::are_equal_are_equal_state_type_function_type_type_function_type;
use crate::functions::seen_set_contains::seen_set_contains;
use crate::records::are_equal_state::AreEqualState;
use crate::records::type_function_negation_type::TypeFunctionNegationType;

#[allow(non_snake_case)]
pub fn are_equal_are_equal_state_type_function_negation_type_type_function_negation_type(
    seen: &mut AreEqualState,
    lhs: &TypeFunctionNegationType,
    rhs: &TypeFunctionNegationType,
) -> bool {
    if seen_set_contains(
        seen,
        lhs as *const TypeFunctionNegationType as *const core::ffi::c_void,
        rhs as *const TypeFunctionNegationType as *const core::ffi::c_void,
    ) {
        return true;
    }

    unsafe {
        are_equal_are_equal_state_type_function_type_type_function_type(
            seen,
            &*lhs.type_id,
            &*rhs.type_id,
        )
    }
}
