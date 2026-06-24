use crate::functions::get_type_alt_j::get_type_id;
use crate::records::blocked_type::BlockedType;
use crate::records::free_type::FreeType;
use crate::records::generic_type::GenericType;
use crate::type_aliases::type_id::TypeId;

pub fn tyvar_index(ty: TypeId) -> i32 {
    unsafe {
        if let Some(gtv) = get_type_id::<GenericType>(ty).as_ref() {
            gtv.index
        } else if let Some(ftv) = get_type_id::<FreeType>(ty).as_ref() {
            ftv.index
        } else if let Some(btv) = get_type_id::<BlockedType>(ty).as_ref() {
            btv.index
        } else {
            0
        }
    }
}
