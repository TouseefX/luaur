use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_prim::is_prim;
use crate::records::primitive_type::PrimitiveType;
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::records::type_iterator::TypeIterator;
use crate::records::union_type::UnionType;
use crate::type_aliases::singleton_variant::SingletonVariantMember;
use crate::type_aliases::type_id::TypeId;

pub fn is_string(ty: TypeId) -> bool {
    unsafe {
        let ty = follow_type_id(ty);

        if is_prim(ty, PrimitiveType::String) {
            return true;
        }

        let st = get_type_id::<SingletonType>(ty);
        if !st.is_null() {
            let st = &*st;
            if StringSingleton::get_if(&st.variant).is_some() {
                return true;
            }
        }

        let utv = get_type_id::<UnionType>(ty);
        if !utv.is_null() {
            // C++ `std::all_of(begin(utv), end(utv), isString)` — iterate via the
            // UnionTypeIterator, which follows + flattens nested unions and carries
            // a seen-set to skip cycles. A raw recursion over `utv.options` stack-
            // overflows on a structurally self-referential union.
            let mut it = TypeIterator::<UnionType>::type_iterator_type(utv as *const UnionType);
            let end_it = TypeIterator::<UnionType>::type_iterator_default();
            while it.operator_ne(&end_it) {
                let option = it.operator_deref();
                it.operator_inc();
                if !is_string(option) {
                    return false;
                }
            }
            return true;
        }

        false
    }
}
