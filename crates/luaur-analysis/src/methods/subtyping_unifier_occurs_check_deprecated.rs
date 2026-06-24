use crate::enums::occurs_check_result::OccursCheckResult;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::records::error_type_pack::ErrorTypePack;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::subtyping_unifier::SubtypingUnifier;
use crate::records::type_pack::TypePack;
use crate::type_aliases::error_type_pack::ErrorTypePack as ErrorTypePackAlias;
use crate::type_aliases::free_type_pack::FreeTypePack as FreeTypePackAlias;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl SubtypingUnifier {
    pub fn occurs_check_deprecated(
        &self,
        needle: TypePackId,
        haystack: TypePackId,
    ) -> OccursCheckResult {
        let needle_followed = unsafe { follow_type_pack_id(needle) };
        let haystack_followed = unsafe { follow_type_pack_id(haystack) };

        if !unsafe {
            crate::functions::get_mutable_type_pack::get_mutable_type_pack_id::<ErrorTypePackAlias>(
                needle_followed,
            )
        }
        .is_null()
        {
            return OccursCheckResult::Pass;
        }

        if unsafe {
            crate::functions::get_mutable_type_pack::get_mutable_type_pack_id::<FreeTypePackAlias>(
                needle_followed,
            )
        }
        .is_null()
        {
            LUAU_ASSERT!(false, "Expected needle pack to be free");
        }

        let mut current_haystack = haystack_followed;
        while unsafe {
            crate::functions::get_mutable_type_pack::get_mutable_type_pack_id::<ErrorTypePackAlias>(
                current_haystack,
            )
        }
        .is_null()
        {
            if needle_followed == current_haystack {
                return OccursCheckResult::Fail;
            }

            let pack_ptr = unsafe {
                crate::functions::get_type_pack::get_type_pack_id::<TypePack>(current_haystack)
            };
            if !pack_ptr.is_null() {
                let tail_opt = unsafe { (*pack_ptr).tail };
                if let Some(tail) = tail_opt {
                    current_haystack = unsafe { follow_type_pack_id(tail) };
                    continue;
                }
            }

            break;
        }

        OccursCheckResult::Pass
    }
}
