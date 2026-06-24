use crate::functions::are_equal_type_function_runtime_alt_m::are_equal_are_equal_state_type_function_type_type_function_type;
use crate::functions::seen_set_contains::seen_set_contains;
use crate::records::are_equal_state::AreEqualState;
use crate::records::type_function_intersection_type::TypeFunctionIntersectionType;

#[allow(non_snake_case)]
pub fn are_equal_are_equal_state_type_function_intersection_type_type_function_intersection_type(
    seen: &mut AreEqualState,
    lhs: &TypeFunctionIntersectionType,
    rhs: &TypeFunctionIntersectionType,
) -> bool {
    if seen_set_contains(
        seen,
        lhs as *const TypeFunctionIntersectionType as *const core::ffi::c_void,
        rhs as *const TypeFunctionIntersectionType as *const core::ffi::c_void,
    ) {
        return true;
    }

    if lhs.components.len() != rhs.components.len() {
        return false;
    }

    for i in 0..lhs.components.len() {
        let l: crate::type_aliases::type_function_type_id::TypeFunctionTypeId = lhs.components[i];
        let r: crate::type_aliases::type_function_type_id::TypeFunctionTypeId = rhs.components[i];

        if !are_equal_are_equal_state_type_function_type_type_function_type(
            seen,
            unsafe { &*l },
            unsafe { &*r },
        ) {
            return false;
        }
    }

    true
}
