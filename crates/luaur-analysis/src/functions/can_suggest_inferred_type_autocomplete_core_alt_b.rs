use crate::functions::can_suggest_inferred_type_autocomplete_core::can_suggest_inferred_type;
use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::generic_type_pack::GenericTypePack;
use crate::type_aliases::error_type_pack::ErrorTypePack;
use crate::type_aliases::type_pack_id::TypePackId;

/// C++ `static bool canSuggestInferredType(TypePackId ty)`.
pub fn can_suggest_inferred_type_type_pack_id(ty: TypePackId) -> bool {
    unsafe {
        let ty = follow_type_pack_id(ty);

        if !get_type_pack_id::<ErrorTypePack>(ty).is_null()
            || !get_type_pack_id::<GenericTypePack>(ty).is_null()
            || !get_type_pack_id::<FreeTypePack>(ty).is_null()
        {
            return false;
        }

        let (head, _tail) = flatten_type_pack_id(ty);

        for head_ty in head {
            if !can_suggest_inferred_type(head_ty) {
                return false;
            }
        }

        true
    }
}
