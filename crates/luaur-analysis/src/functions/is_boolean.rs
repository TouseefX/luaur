use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_prim::is_prim;
use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::primitive_type::PrimitiveType;
use crate::records::singleton_type::SingletonType;
use crate::records::union_type::UnionType;
use crate::type_aliases::singleton_variant::SingletonVariantMember;
use crate::type_aliases::type_id::TypeId;

pub fn is_boolean(ty: TypeId) -> bool {
    unsafe {
        if is_prim(ty, PrimitiveType::Boolean) {
            return true;
        }

        let stv = get_type_id::<SingletonType>(follow_type_id(ty));
        if !stv.is_null() && BooleanSingleton::get_if(&(*stv).variant).is_some() {
            return true;
        }

        let utv = get_type_id::<UnionType>(follow_type_id(ty));
        if !utv.is_null() {
            return (*utv).options.iter().all(|&opt| is_boolean(opt));
        }

        false
    }
}
