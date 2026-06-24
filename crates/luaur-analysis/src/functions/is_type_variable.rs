use crate::functions::get_type_alt_j::get_type_id;
use crate::records::blocked_type::BlockedType;
use crate::records::free_type::FreeType;
use crate::records::generic_type::GenericType;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::type_aliases::type_id::TypeId;

pub fn is_type_variable(ty: TypeId) -> bool {
    unsafe {
        !get_type_id::<FreeType>(ty).is_null()
            || !get_type_id::<GenericType>(ty).is_null()
            || !get_type_id::<BlockedType>(ty).is_null()
            || !get_type_id::<PendingExpansionType>(ty).is_null()
    }
}
