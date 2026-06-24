use crate::enums::normalization_result::NormalizationResult;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::intersection_type::IntersectionType;
use crate::records::metatable_type::MetatableType;
use crate::records::never_type::NeverType;
use crate::records::normalizer::Normalizer;
use crate::records::table_type::TableType;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

struct RecursionCountGuard {
    count: *mut i32,
}

impl RecursionCountGuard {
    fn new(count: *mut i32) -> Self {
        unsafe {
            *count += 1;
        }
        Self { count }
    }
}

impl Drop for RecursionCountGuard {
    fn drop(&mut self) {
        unsafe {
            debug_assert!(*self.count > 0);
            *self.count -= 1;
        }
    }
}

impl Normalizer {
    pub fn is_inhabited_type_id_set_type_id(
        &mut self,
        ty: TypeId,
        seen: &mut DenseHashSet<TypeId>,
    ) -> NormalizationResult {
        let _rc =
            RecursionCountGuard::new(unsafe { &mut (*self.shared_state).counters.recursion_count });

        if !self.within_resource_limits() {
            return NormalizationResult::HitLimits;
        }

        self.consume_fuel();

        let ty = unsafe { follow_type_id(ty) };

        if !unsafe { get_type_id::<NeverType>(ty).is_null() } {
            return NormalizationResult::False;
        }

        if unsafe { get_type_id::<IntersectionType>(ty).is_null() }
            && unsafe { get_type_id::<UnionType>(ty).is_null() }
            && unsafe { get_type_id::<TableType>(ty).is_null() }
            && unsafe { get_type_id::<MetatableType>(ty).is_null() }
        {
            return NormalizationResult::True;
        }

        if seen.contains(&ty) {
            return NormalizationResult::True;
        }

        seen.insert(ty);

        if let Some(ttv) = unsafe { get_type_id::<TableType>(ty).as_ref() } {
            for (_k, prop) in &ttv.props {
                if self.use_new_luau_solver() {
                    if let Some(ty) = prop.read_ty {
                        let res = self.is_inhabited_type_id_set_type_id(ty, seen);
                        if res != NormalizationResult::True {
                            return res;
                        }
                    }
                } else {
                    let res = self.is_inhabited_type_id_set_type_id(prop.read_ty.unwrap(), seen);
                    if res != NormalizationResult::True {
                        return res;
                    }
                }
            }
            return NormalizationResult::True;
        }

        if let Some(mtv) = unsafe { get_type_id::<MetatableType>(ty).as_ref() } {
            let res = self.is_inhabited_type_id_set_type_id(mtv.table, seen);
            if res != NormalizationResult::True {
                return res;
            }
            return self.is_inhabited_type_id_set_type_id(mtv.metatable, seen);
        }

        let norm = self.normalize(ty);
        self.is_inhabited_normalized_type_set_type_id(norm.as_ref(), seen)
    }
}
