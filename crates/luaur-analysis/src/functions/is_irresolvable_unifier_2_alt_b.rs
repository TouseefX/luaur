use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::blocked_type_pack::BlockedTypePack;
use crate::records::type_function_instance_type_pack::TypeFunctionInstanceTypePack;
use crate::type_aliases::type_pack_id::TypePackId;

// returns `true` if `tp` is irresolvable and should be added to `incompleteSubtypes`.
pub fn is_irresolvable(tp: TypePackId) -> bool {
    unsafe {
        !get_type_pack_id::<BlockedTypePack>(tp).is_null()
            || !get_type_pack_id::<TypeFunctionInstanceTypePack>(tp).is_null()
    }
}
