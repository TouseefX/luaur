use crate::functions::are_equal_type_function_runtime_alt_m::are_equal_are_equal_state_type_function_type_type_function_type;
use crate::records::are_equal_state::AreEqualState;
use crate::records::type_function_type::TypeFunctionType;
use crate::records::type_function_type_pack::TypeFunctionTypePack;

#[allow(non_snake_case)]
pub fn are_equal_are_equal_state_type_function_type_pack_type_function_type_pack(
    seen: &mut AreEqualState,
    lhs: &TypeFunctionTypePack,
    rhs: &TypeFunctionTypePack,
) -> bool {
    if lhs.head.len() != rhs.head.len() {
        return false;
    }

    for i in 0..lhs.head.len() {
        let l: *const TypeFunctionType = lhs.head[i];
        let r: *const TypeFunctionType = rhs.head[i];

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
