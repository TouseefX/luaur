//! @interface-stub
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::merge::merge;
use crate::records::type_checker::TypeChecker;
use crate::records::union_type::UnionType;
use crate::type_aliases::refinement_map::RefinementMap;
use std::collections::HashSet;

impl TypeChecker {
    pub fn merge(&mut self, l: &mut RefinementMap, r: &RefinementMap) {
        let this = self as *mut TypeChecker;

        merge(l, r, &|a, b| {
            let mut set = HashSet::new();

            unsafe {
                let followed_a = follow_type_id(a);
                let utv = get_type_id::<UnionType>(followed_a);
                if !utv.is_null() {
                    for option in &(*utv).options {
                        set.insert(*option);
                    }
                } else {
                    set.insert(a);
                }

                let followed_b = follow_type_id(b);
                let utv = get_type_id::<UnionType>(followed_b);
                if !utv.is_null() {
                    for option in &(*utv).options {
                        set.insert(*option);
                    }
                } else {
                    set.insert(b);
                }
            }

            let options: alloc::vec::Vec<_> = set.into_iter().collect();
            if options.len() == 1 {
                options[0]
            } else {
                unsafe { (*this).add_type(&UnionType { options }) }
            }
        });
    }
}
