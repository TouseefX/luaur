use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get;
use crate::records::negation_type::NegationType;

/// C++ `bool isTruthyType_DEPRECATED(TypeId ty)`.
pub fn is_truthy_type_deprecated(ty: crate::type_aliases::type_id::TypeId) -> bool {
    unsafe {
        let ty = follow_type_id(ty);
        let nt = get::<NegationType>(ty);
        if nt.is_null() {
            return false;
        }

        let nt = &*nt;
        crate::functions::is_falsy_type_deprecated::is_falsy_type_deprecated(nt.ty)
    }
}
