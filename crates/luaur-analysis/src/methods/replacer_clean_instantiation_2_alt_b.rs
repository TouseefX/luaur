use crate::records::replacer::Replacer;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Replacer {
    pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
        let res = unsafe { (*self.replacement_packs).find(&tp) }
            .expect("TypePackId not found in replacement_packs");
        LUAU_ASSERT!(!res.is_null());
        let cleaned = *res;
        self.base.dont_traverse_into_type_pack_id(cleaned);
        cleaned
    }
}
