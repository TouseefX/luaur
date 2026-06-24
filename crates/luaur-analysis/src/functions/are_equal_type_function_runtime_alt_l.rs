use crate::functions::seen_set_contains::seen_set_contains;
use crate::records::are_equal_state::AreEqualState;
use crate::records::type_function_extern_type::TypeFunctionExternType;

#[allow(non_snake_case)]
pub fn are_equal_are_equal_state_type_function_extern_type_type_function_extern_type(
    seen: &mut AreEqualState,
    lhs: &TypeFunctionExternType,
    rhs: &TypeFunctionExternType,
) -> bool {
    if seen_set_contains(
        seen,
        lhs as *const TypeFunctionExternType as *const core::ffi::c_void,
        rhs as *const TypeFunctionExternType as *const core::ffi::c_void,
    ) {
        return true;
    }

    lhs.extern_ty == rhs.extern_ty
}
