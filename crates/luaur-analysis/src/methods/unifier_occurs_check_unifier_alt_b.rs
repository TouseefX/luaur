//! Source: `Analysis/src/Unifier.cpp` (Unifier::occursCheck(DenseHashSet<TypeId>&,...), L2644-2687)
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::records::free_type::FreeType;
use crate::records::intersection_type::IntersectionType;
use crate::records::unifier::Unifier;
use crate::records::union_type::UnionType;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl Unifier {
    /// `bool Unifier::occursCheck(DenseHashSet<TypeId>& seen, TypeId needle, TypeId haystack)`
    pub fn occurs_check_dense_hash_set_type_id_type_id_type_id(
        &mut self,
        seen: &mut DenseHashSet<TypeId>,
        mut needle: TypeId,
        mut haystack: TypeId,
    ) -> bool {
        let mut occurrence = false;

        needle = self.log.follow_type_id(needle);
        haystack = self.log.follow_type_id(haystack);

        if seen.find(&haystack).is_some() {
            return false;
        }

        seen.insert(haystack);

        if !unsafe { get_mutable_type_id::<ErrorType>(needle) }.is_null() {
            return false;
        }

        if unsafe { get_mutable_type_id::<FreeType>(needle) }.is_null() {
            self.ice_string("Expected needle to be free");
        }

        if needle == haystack {
            return true;
        }

        if !unsafe { get_mutable_type_id::<FreeType>(haystack) }.is_null() {
            return false;
        } else if let Some(a) = unsafe { get_mutable_type_id::<UnionType>(haystack).as_ref() } {
            let options = a.options.clone();
            for ty in options {
                if self.occurs_check_dense_hash_set_type_id_type_id_type_id(seen, needle, ty) {
                    occurrence = true;
                }
            }
        } else if let Some(a) =
            unsafe { get_mutable_type_id::<IntersectionType>(haystack).as_ref() }
        {
            let parts = a.parts.clone();
            for ty in parts {
                if self.occurs_check_dense_hash_set_type_id_type_id_type_id(seen, needle, ty) {
                    occurrence = true;
                }
            }
        }

        occurrence
    }
}
