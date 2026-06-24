use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_nil::is_nil;
use crate::records::type_arena::TypeArena;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;

pub fn try_strip_union_from_nil(arena: &mut TypeArena, ty: TypeId) -> Option<TypeId> {
    unsafe {
        if !get_type_id::<UnionType>(ty).is_null() {
            let utv = &*get_type_id::<UnionType>(ty);

            if !utv.options.iter().any(|option| is_nil(*option)) {
                return Some(ty);
            }

            let mut result = Vec::new();

            for option in &utv.options {
                if !is_nil(*option) {
                    result.push(*option);
                }
            }

            if result.is_empty() {
                return None;
            }

            return if result.len() == 1 {
                Some(result[0])
            } else {
                Some(arena.add_type(UnionType { options: result }))
            };
        }

        None
    }
}
