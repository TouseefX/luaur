use crate::records::r#type::Type;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Type {
    pub fn operator_assign_type_item_mut(&mut self, rhs: Type) -> &mut Self {
        LUAU_ASSERT!(self.owning_arena == rhs.owning_arena);
        LUAU_ASSERT!(!rhs.persistent);

        self.reassign(&rhs);

        self
    }
}
