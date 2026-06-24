use crate::enums::occurs_check_result::OccursCheckResult;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::blocked_type_pack::BlockedTypePack;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::type_pack::TypePack;
use crate::type_aliases::error_type_pack::ErrorTypePack;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn occurs_check_type_pack_id_type_pack_id(
    needle: TypePackId,
    haystack: TypePackId,
) -> OccursCheckResult {
    unsafe {
        let needle = follow_type_pack_id(needle);
        let mut haystack = follow_type_pack_id(haystack);

        LUAU_ASSERT!(
            !get_type_pack_id::<FreeTypePack>(needle).is_null()
                || !get_type_pack_id::<BlockedTypePack>(needle).is_null()
        );

        if !get_type_pack_id::<ErrorTypePack>(needle).is_null() {
            return OccursCheckResult::Pass;
        }

        while get_type_pack_id::<ErrorTypePack>(haystack).is_null() {
            if needle == haystack {
                return OccursCheckResult::Fail;
            }

            let a = get_type_pack_id::<TypePack>(haystack);
            if !a.is_null() {
                if let Some(tail) = (*a).tail {
                    haystack = follow_type_pack_id(tail);
                    continue;
                }
            }

            break;
        }

        OccursCheckResult::Pass
    }
}
