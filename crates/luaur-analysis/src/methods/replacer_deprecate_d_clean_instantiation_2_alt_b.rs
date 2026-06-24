use crate::records::replacer_deprecated::ReplacerDeprecated;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl ReplacerDeprecated {
    pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
        let res = self
            .replacement_packs
            .find(&tp)
            .expect("TypePackId not found in replacement_packs");
        LUAU_ASSERT!(!res.is_null());
        let cleaned = *res;
        self.base.dont_traverse_into_type_pack_id(cleaned);
        cleaned
    }
}
