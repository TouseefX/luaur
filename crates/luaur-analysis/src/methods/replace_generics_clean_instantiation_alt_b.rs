use crate::records::free_type_pack::FreeTypePack;
use crate::records::replace_generics::ReplaceGenerics;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl ReplaceGenerics {
    pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
        LUAU_ASSERT!(self.is_dirty_type_pack_id(tp));
        let mut pack = FreeTypePack::new(self.level);
        pack.scope = self.scope;
        self.base.add_type_pack(pack)
    }
}
