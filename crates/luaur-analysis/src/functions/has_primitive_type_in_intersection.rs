use crate::functions::flatten_intersection::flatten_intersection;
use crate::functions::follow_type::follow_type_id;
use crate::functions::is_prim::is_prim;
use crate::records::primitive_type::Type;
use crate::type_aliases::type_id::TypeId;

pub fn has_primitive_type_in_intersection(ty: TypeId, prim_ty: Type) -> bool {
    let tf = unsafe { follow_type_id(ty) };
    if is_prim(tf, prim_ty) {
        return true;
    }

    for t in flatten_intersection(tf) {
        let followed = unsafe { follow_type_id(t) };
        if is_prim(followed, prim_ty) {
            return true;
        }
    }

    false
}
