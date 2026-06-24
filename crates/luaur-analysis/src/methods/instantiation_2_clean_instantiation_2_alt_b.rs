use crate::records::instantiation_2::Instantiation2;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Instantiation2 {
    pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
        let res = self
            .generic_pack_substitutions
            .find(&tp)
            .expect("TypePackId not found in generic_pack_substitutions");
        LUAU_ASSERT!(!res.is_null());
        let cleaned = *res;
        self.base.dont_traverse_into_type_pack_id(cleaned);
        cleaned
    }
}
