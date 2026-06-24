use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::any_type::AnyType;
use crate::records::free_type::FreeType;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::type_id::TypeId;

/// C++ `static bool isUndecidable(TypeId ty)`.
pub fn is_undecidable(ty: TypeId) -> bool {
    unsafe {
        let ty = follow_type_id(ty);
        !get_type_id::<AnyType>(ty).is_null()
            || !get_type_id::<ErrorType>(ty).is_null()
            || !get_type_id::<FreeType>(ty).is_null()
    }
}
