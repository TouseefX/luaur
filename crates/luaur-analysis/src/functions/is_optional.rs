use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_nil::is_nil;
use crate::records::any_type::AnyType;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::type_id::TypeId;
use std::collections::HashSet;

pub fn is_optional(ty: TypeId) -> bool {
    unsafe {
        let mut seen = HashSet::<TypeId>::new();
        let mut stack = vec![ty];

        while let Some(ty) = stack.pop() {
            let ty = follow_type_id(ty);
            if !seen.insert(ty) {
                continue;
            }

            if is_nil(ty)
                || !get_type_id::<AnyType>(ty).is_null()
                || !get_type_id::<UnknownType>(ty).is_null()
            {
                return true;
            }

            let utv = get_type_id::<UnionType>(ty);
            if !utv.is_null() {
                stack.extend((*utv).options.iter().copied());
            }
        }

        false
    }
}
