use crate::functions::follow_type_id::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub fn occurs(mut haystack: TypeId, needle: TypeId, seen: &mut DenseHashSet<TypeId>) -> bool {
    haystack = unsafe { follow_type_id(haystack) };

    if needle == haystack {
        return true;
    }
    if seen.contains(&haystack) {
        return false;
    }
    seen.insert(haystack);

    if let Some(ut) = unsafe { get_type_id::<UnionType>(haystack).as_ref() } {
        for &option in &ut.options {
            if occurs(option, needle, seen) {
                return true;
            }
        }
    }
    false
}
