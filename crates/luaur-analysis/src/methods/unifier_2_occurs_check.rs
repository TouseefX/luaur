//! Source: `Analysis/src/Unifier2.cpp:825-871` — `Unifier2::occursCheck(DenseHashSet<TypeId>&, TypeId, TypeId)`.

use crate::enums::occurs_check_result::OccursCheckResult;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::error_type::ErrorType;
use crate::records::free_type::FreeType;
use crate::records::intersection_type::IntersectionType;
use crate::records::recursion_limiter::RecursionLimiter;
use crate::records::unifier_2::Unifier2;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl Unifier2 {
    pub fn occurs_check(
        &mut self,
        seen: &mut DenseHashSet<TypeId>,
        needle: TypeId,
        haystack: TypeId,
    ) -> OccursCheckResult {
        let mut _ra = RecursionLimiter {
            base: unsafe { core::mem::zeroed() },
            native_stack_guard: unsafe { core::mem::zeroed() },
        };
        _ra.recursion_limiter_recursion_limiter(
            "Unifier2::occursCheck",
            &mut self.recursion_count as *mut i32 as *mut core::ffi::c_int,
            self.recursion_limit as core::ffi::c_int,
        );

        let mut occurrence = OccursCheckResult::Pass;

        let needle = unsafe { follow_type_id(needle) };
        let haystack = unsafe { follow_type_id(haystack) };

        if seen.find(&haystack).is_some() {
            return OccursCheckResult::Pass;
        }

        seen.insert(haystack);

        if !unsafe { get_type_id::<ErrorType>(needle) }.is_null() {
            return OccursCheckResult::Pass;
        }

        if unsafe { get_type_id::<FreeType>(needle) }.is_null() {
            unsafe { (*self.ice.as_ptr()).ice_string("Expected needle to be free") };
        }

        if needle == haystack {
            return OccursCheckResult::Fail;
        }

        let haystack_free = unsafe { get_type_id::<FreeType>(haystack) };
        if !haystack_free.is_null() {
            let lower = unsafe { (*haystack_free).lower_bound };
            let upper = unsafe { (*haystack_free).upper_bound };
            if self.occurs_check(seen, needle, lower) == OccursCheckResult::Fail {
                occurrence = OccursCheckResult::Fail;
            }
            if self.occurs_check(seen, needle, upper) == OccursCheckResult::Fail {
                occurrence = OccursCheckResult::Fail;
            }
        } else {
            let ut = unsafe { get_type_id::<UnionType>(haystack) };
            if !ut.is_null() {
                let options: alloc::vec::Vec<TypeId> = unsafe { (*ut).options.clone() };
                for ty in options {
                    if self.occurs_check(seen, needle, ty) == OccursCheckResult::Fail {
                        occurrence = OccursCheckResult::Fail;
                    }
                }
            } else {
                let it = unsafe { get_type_id::<IntersectionType>(haystack) };
                if !it.is_null() {
                    let parts: alloc::vec::Vec<TypeId> = unsafe { (*it).parts.clone() };
                    for ty in parts {
                        if self.occurs_check(seen, needle, ty) == OccursCheckResult::Fail {
                            occurrence = OccursCheckResult::Fail;
                        }
                    }
                }
            }
        }

        occurrence
    }
}
