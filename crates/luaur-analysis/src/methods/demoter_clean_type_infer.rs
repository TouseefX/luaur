use crate::functions::get_type_alt_j::get_type_id;
use crate::records::demoter::Demoter;
use crate::records::free_type::FreeType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Demoter {
    pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
        let ftv = unsafe { get_type_id::<FreeType>(ty) };
        LUAU_ASSERT!(!ftv.is_null());
        let ftv_ref = unsafe { &*ftv };
        let level = ftv_ref.level;
        let demoted_level = self.demoted_level(level);
        let arena = unsafe { &mut *self.arena };
        arena
            .fresh_type_not_null_builtin_types_type_level(unsafe { &*self.builtins }, demoted_level)
    }
}
