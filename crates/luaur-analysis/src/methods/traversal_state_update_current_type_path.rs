use crate::functions::follow_type::follow_type_id;
use crate::records::traversal_state::TraversalState;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TraversalState {
    pub fn update_current_type_id(&mut self, ty: TypeId) {
        LUAU_ASSERT!(!ty.is_null());
        self.current =
            crate::type_aliases::type_or_pack::TypeOrPack::V0(unsafe { follow_type_id(ty) });
    }
}
