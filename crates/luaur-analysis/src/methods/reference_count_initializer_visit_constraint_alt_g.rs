use crate::records::blocked_type_pack::BlockedTypePack;
use crate::records::reference_count_initializer::ReferenceCountInitializer;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::FFlag;

impl ReferenceCountInitializer {
    pub fn visit_type_pack_id_blocked_type_pack(
        &mut self,
        tp: TypePackId,
        _blocked_type_pack: &BlockedTypePack,
    ) -> bool {
        if FFlag::LuauConstraintGraph.get() {
            LUAU_ASSERT!(self.mutated_type_packs.is_null() == false);
            unsafe {
                (*self.mutated_type_packs).insert(tp);
            }
        }
        true
    }
}
