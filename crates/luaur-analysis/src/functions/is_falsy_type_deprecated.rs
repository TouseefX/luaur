use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::primitive_type::PrimitiveType;
use crate::records::singleton_type::SingletonType;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::variant::Variant2;

pub fn is_falsy_type_deprecated(ty: TypeId) -> bool {
    unsafe {
        let ty = follow_type_id(ty);

        let ut = get_type_id::<UnionType>(ty);
        if ut.is_null() {
            return false;
        }

        let utv = &*ut;
        if utv.options.len() != 2 {
            return false;
        }

        let mut has_false = false;
        let mut has_nil = false;

        for option in utv.options.iter() {
            let t = follow_type_id(*option);

            let pt = get_type_id::<PrimitiveType>(t);
            if !pt.is_null() {
                let ptv = &*pt;
                if ptv.r#type == PrimitiveType::NilType {
                    has_nil = true;
                } else {
                    return false;
                }
            } else {
                let st = get_type_id::<SingletonType>(t);
                if !st.is_null() {
                    let stv = &*st;
                    if stv.variant == Variant2::V0(BooleanSingleton::new(false)) {
                        has_false = true;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        has_false && has_nil
    }
}
