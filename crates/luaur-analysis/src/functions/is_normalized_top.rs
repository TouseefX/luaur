use crate::functions::get_type_alt_j::get_type_id;
use crate::records::any_type::AnyType;
use crate::records::never_type::NeverType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::type_id::TypeId;

pub fn is_normalized_top(ty: TypeId) -> bool {
    unsafe {
        !get_type_id::<NeverType>(ty).is_null()
            || !get_type_id::<AnyType>(ty).is_null()
            || !get_type_id::<UnknownType>(ty).is_null()
    }
}
