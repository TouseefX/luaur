use crate::functions::begin_type_pack::begin;
use crate::functions::end_type_pack::end;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::type_pack_id::TypePackId;

/// C++ `static bool allowsNoReturnValues(const TypePackId tp)`.
pub fn allows_no_return_values(tp: TypePackId) -> bool {
    unsafe {
        let mut it = begin(tp);
        let end_it = end(tp);

        while it.operator_ne(&end_it) {
            let ty: crate::type_aliases::type_id::TypeId = *it.operator_deref();
            let followed_ty = follow_type_id(ty);

            if get_type_id::<ErrorType>(followed_ty).is_null() {
                return false;
            }

            it.operator_inc();
        }

        true
    }
}
