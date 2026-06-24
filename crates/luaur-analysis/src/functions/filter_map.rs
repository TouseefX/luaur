use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_id_predicate::TypeIdPredicate;
use alloc::vec::Vec;
use std::collections::HashSet;

pub fn filter_map(type_: TypeId, predicate: TypeIdPredicate) -> Vec<TypeId> {
    let type_ = unsafe { follow_type_id(type_) };

    unsafe {
        if !get_type_id::<UnionType>(type_).is_null() {
            let utv = get_type_id::<UnionType>(type_);
            let mut options: HashSet<TypeId> = HashSet::new();

            for &option in (*utv).options.iter() {
                let followed_option = follow_type_id(option);
                if let Some(out) = predicate(followed_option) {
                    options.insert(out);
                }
            }

            options.into_iter().collect()
        } else if let Some(out) = predicate(type_) {
            vec![out]
        } else {
            Vec::new()
        }
    }
}
