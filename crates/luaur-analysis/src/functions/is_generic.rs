use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::function_type::FunctionType;
use crate::type_aliases::type_id::TypeId;

pub fn is_generic(ty: TypeId) -> bool {
    unsafe {
        let followed = follow_type_id(ty);
        let ftv = get_type_id::<FunctionType>(followed);
        if ftv.is_null() {
            false
        } else {
            let ftv_ref = &*ftv;
            !ftv_ref.generics.is_empty() || !ftv_ref.generic_packs.is_empty()
        }
    }
}
