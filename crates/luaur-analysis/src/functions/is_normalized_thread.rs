use crate::functions::get_type_alt_j::get_type_id;
use crate::records::never_type::NeverType;
use crate::records::primitive_type::PrimitiveType;
use crate::type_aliases::type_id::TypeId;

pub fn is_normalized_thread(ty: TypeId) -> bool {
    unsafe {
        if get_type_id::<NeverType>(ty).is_null() {
            return true;
        } else if let Some(ptv) = get_type_id::<PrimitiveType>(ty).as_ref() {
            return ptv.r#type == PrimitiveType::Thread;
        } else {
            return false;
        }
    }
}
