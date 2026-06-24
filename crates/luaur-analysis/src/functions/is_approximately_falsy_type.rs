use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::primitive_type::PrimitiveType;
use crate::records::singleton_type::SingletonType;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::variant::Variant2;

pub fn is_approximately_falsy_type(ty: TypeId) -> bool {
    unsafe {
        let ty = follow_type_id(ty);

        let mut seen_nil = false;
        let mut seen_false = false;

        if let Some(utv) = get_type_id::<UnionType>(ty).as_ref() {
            for option in utv.options.iter() {
                let option = follow_type_id(*option);

                if let Some(ptv) = get_type_id::<PrimitiveType>(option).as_ref() {
                    if ptv.r#type == PrimitiveType::NilType {
                        seen_nil = true;
                    } else {
                        return false;
                    }
                } else if let Some(stv) = get_type_id::<SingletonType>(option).as_ref() {
                    if stv.variant == Variant2::V0(BooleanSingleton::new(false)) {
                        seen_false = true;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        } else {
            return false;
        }

        seen_false && seen_nil
    }
}
