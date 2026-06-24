use crate::functions::get_type_alt_j::get_type_id;
use crate::records::never_type::NeverType;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::type_id::TypeId;

pub fn is_normalized_error(ty: TypeId) -> bool {
    unsafe { !get_type_id::<NeverType>(ty).is_null() || !get_type_id::<ErrorType>(ty).is_null() }
}
