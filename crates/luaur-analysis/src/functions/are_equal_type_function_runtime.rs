use crate::functions::seen_set_contains::seen_set_contains;
use crate::records::are_equal_state::AreEqualState;
use crate::records::type_function_boolean_singleton::TypeFunctionBooleanSingleton;
use crate::records::type_function_singleton_type::TypeFunctionSingletonType;
use crate::records::type_function_string_singleton::TypeFunctionStringSingleton;
use luaur_common::records::variant::Variant2;

#[allow(non_snake_case)]
pub fn are_equal_are_equal_state_type_function_singleton_type_type_function_singleton_type(
    seen: &mut AreEqualState,
    lhs: &TypeFunctionSingletonType,
    rhs: &TypeFunctionSingletonType,
) -> bool {
    if seen_set_contains(
        seen,
        lhs as *const TypeFunctionSingletonType as *const core::ffi::c_void,
        rhs as *const TypeFunctionSingletonType as *const core::ffi::c_void,
    ) {
        return true;
    }

    match (&lhs.variant, &rhs.variant) {
        (Variant2::V0(lp), Variant2::V0(rp)) => {
            return lp.value == rp.value;
        }
        (Variant2::V1(lp), Variant2::V1(rp)) => {
            return lp.value == rp.value;
        }
        _ => {}
    }

    false
}
