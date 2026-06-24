use crate::records::replacer::Replacer;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Replacer {
    pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
        let res =
            unsafe { (*self.replacements).find(&ty) }.expect("TypeId not found in replacements");
        LUAU_ASSERT!(!res.is_null());
        let cleaned = *res;
        self.base.dont_traverse_into_type_id(cleaned);
        cleaned
    }
}
