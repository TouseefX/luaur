use crate::functions::get_type_alt_j::get_type_id;
use crate::records::function_type::FunctionType;
use crate::records::normalized_function_type::NormalizedFunctionType;
use crate::type_aliases::error_type::ErrorType;

/// C++ `static bool areNormalizedFunctions(const NormalizedFunctionType& tys)`.
pub fn are_normalized_functions(tys: &NormalizedFunctionType) -> bool {
    for &ty in &tys.parts.order {
        unsafe {
            if get_type_id::<FunctionType>(ty).is_null() && get_type_id::<ErrorType>(ty).is_null() {
                return false;
            }
        }
    }

    true
}
