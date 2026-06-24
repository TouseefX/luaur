use crate::records::type_pack_var::TypePackVar;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypePackVar {
    pub fn operator_assign_type_pack_var(&mut self, rhs: &TypePackVar) -> &mut Self {
        LUAU_ASSERT!(self.owningArena == rhs.owningArena);
        LUAU_ASSERT!(!rhs.persistent);

        self.reassign(rhs);

        self
    }
}
