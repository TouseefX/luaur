use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::primitive_type::PrimitiveType;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;

pub fn is_optional_type(ty: TypeId, builtin_types: *mut BuiltinTypes) -> bool {
    unsafe {
        let ty = follow_type_id(ty);
        let builtin_types_ref = &*builtin_types;

        if ty == builtin_types_ref.nilType
            || ty == builtin_types_ref.anyType
            || ty == builtin_types_ref.unknownType
        {
            return true;
        } else if let Some(ptv) = get_type_id::<PrimitiveType>(ty).as_ref() {
            return ptv.r#type == PrimitiveType::NilType;
        } else if let Some(utv) = get_type_id::<UnionType>(ty).as_ref() {
            for option in utv.options.iter() {
                let option = follow_type_id(*option);

                if option == builtin_types_ref.nilType
                    || option == builtin_types_ref.anyType
                    || option == builtin_types_ref.unknownType
                {
                    return true;
                } else if let Some(ptv) = get_type_id::<PrimitiveType>(option).as_ref() {
                    if ptv.r#type == PrimitiveType::NilType {
                        return true;
                    }
                }
            }
            return false;
        }

        false
    }
}
