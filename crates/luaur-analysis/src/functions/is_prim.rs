use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::primitive_type::{PrimitiveType, Type};
use crate::type_aliases::type_id::TypeId;

pub fn is_prim(ty: TypeId, prim_type: Type) -> bool {
    let followed = unsafe { follow_type_id(ty) };
    let p = unsafe { get_type_id::<PrimitiveType>(followed) };
    if p.is_null() {
        false
    } else {
        unsafe { (*p).r#type == prim_type }
    }
}
