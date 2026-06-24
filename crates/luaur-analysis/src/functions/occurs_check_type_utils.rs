use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::blocked_type::BlockedType;
use crate::records::intersection_type::IntersectionType;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn occurs_check_type_id_type_id(needle: TypeId, haystack: TypeId) -> bool {
    unsafe {
        LUAU_ASSERT!(
            !get_type_id::<BlockedType>(needle).is_null()
                || !get_type_id::<PendingExpansionType>(needle).is_null()
        );

        let haystack = follow_type_id(haystack);

        let check_haystack =
            |haystack: TypeId| -> bool { occurs_check_type_id_type_id(needle, haystack) };

        if needle == haystack {
            true
        } else {
            let ut = get_type_id::<UnionType>(haystack);
            if !ut.is_null() {
                return (*ut).options.iter().any(|&option| check_haystack(option));
            }

            let it = get_type_id::<IntersectionType>(haystack);
            if !it.is_null() {
                return (*it).parts.iter().any(|&part| check_haystack(part));
            }

            false
        }
    }
}
