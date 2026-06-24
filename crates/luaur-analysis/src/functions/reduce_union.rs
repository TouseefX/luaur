use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::any_type::AnyType;
use crate::records::never_type::NeverType;
use crate::records::union_type::UnionType;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;

pub fn reduce_union(types: &[TypeId]) -> Vec<TypeId> {
    let mut result = Vec::new();

    for &t in types {
        let t = unsafe { follow_type_id(t) };

        unsafe {
            if !get_type_id::<NeverType>(t).is_null() {
                continue;
            }

            if !get_type_id::<ErrorType>(t).is_null() || !get_type_id::<AnyType>(t).is_null() {
                return vec![t];
            }

            if let Some(utv) = get_type_id::<UnionType>(t).as_ref() {
                for &ty in (*utv).options.iter() {
                    let ty = follow_type_id(ty);

                    if !get_type_id::<NeverType>(ty).is_null() {
                        continue;
                    }

                    if !get_type_id::<ErrorType>(ty).is_null()
                        || !get_type_id::<AnyType>(ty).is_null()
                    {
                        return vec![ty];
                    }

                    if !result.contains(&ty) {
                        result.push(ty);
                    }
                }
            } else if !result.contains(&t) {
                result.push(t);
            }
        }
    }

    result
}
