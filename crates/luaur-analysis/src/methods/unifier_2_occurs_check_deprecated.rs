//! Source: `Analysis/src/Unifier2.cpp:873-906` —
//! `Unifier2::occursCheck_DEPRECATED(DenseHashSet<TypePackId>&, TypePackId, TypePackId)`.

use crate::enums::occurs_check_result::OccursCheckResult;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_mutable_type_pack::get_mutable_type_pack_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::recursion_limiter::RecursionLimiter;
use crate::records::type_pack::TypePack;
use crate::records::unifier_2::Unifier2;
use crate::type_aliases::error_type_pack::ErrorTypePack;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl Unifier2 {
    pub fn occurs_check_deprecated(
        &mut self,
        seen: &mut DenseHashSet<TypePackId>,
        needle: TypePackId,
        mut haystack: TypePackId,
    ) -> OccursCheckResult {
        let needle = unsafe { follow_type_pack_id(needle) };
        haystack = unsafe { follow_type_pack_id(haystack) };

        if seen.find(&haystack).is_some() {
            return OccursCheckResult::Pass;
        }

        seen.insert(haystack);

        if !unsafe { get_mutable_type_pack_id::<ErrorTypePack>(needle) }.is_null() {
            return OccursCheckResult::Pass;
        }

        if unsafe { get_mutable_type_pack_id::<FreeTypePack>(needle) }.is_null() {
            unsafe { (*self.ice.as_ptr()).ice_string("Expected needle pack to be free") };
        }

        let mut _ra = RecursionLimiter {
            base: unsafe { core::mem::zeroed() },
            native_stack_guard: unsafe { core::mem::zeroed() },
        };
        _ra.recursion_limiter_recursion_limiter(
            "Unifier2::occursCheck",
            &mut self.recursion_count as *mut i32 as *mut core::ffi::c_int,
            self.recursion_limit as core::ffi::c_int,
        );

        while unsafe { get_mutable_type_pack_id::<ErrorTypePack>(haystack) }.is_null() {
            if needle == haystack {
                return OccursCheckResult::Fail;
            }

            let a = unsafe { get_type_pack_id::<TypePack>(haystack) };
            if !a.is_null() {
                if let Some(tail) = unsafe { (*a).tail } {
                    haystack = unsafe { follow_type_pack_id(tail) };
                    continue;
                }
            }

            break;
        }

        OccursCheckResult::Pass
    }
}
