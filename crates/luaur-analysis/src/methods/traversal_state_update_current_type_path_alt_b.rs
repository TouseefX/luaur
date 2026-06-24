use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::records::traversal_state::TraversalState;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TraversalState {
    pub fn update_current_type_pack_id(&mut self, tp: TypePackId) {
        LUAU_ASSERT!(!tp.is_null());
        self.current =
            crate::type_aliases::type_or_pack::TypeOrPack::V1(unsafe { follow_type_pack_id(tp) });
    }
}
