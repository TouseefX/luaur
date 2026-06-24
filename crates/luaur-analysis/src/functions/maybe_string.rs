use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_prim::is_prim;
use crate::records::any_type::AnyType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;

pub fn maybe_string(ty: TypeId) -> bool {
    let ty = unsafe { follow_type_id(ty) };

    if is_prim(ty, PrimitiveType::String) || unsafe { !get_type_id::<AnyType>(ty).is_null() } {
        return true;
    }

    if let Some(utv) = unsafe { get_type_id::<UnionType>(ty).as_ref() } {
        for &part in &utv.options {
            if maybe_string(part) {
                return true;
            }
        }
    }

    false
}
