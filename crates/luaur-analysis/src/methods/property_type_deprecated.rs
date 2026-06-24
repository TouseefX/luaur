use crate::records::property_type::Property;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Property {
    pub fn type_deprecated(&self) -> TypeId {
        LUAU_ASSERT!(self.read_ty.is_some());
        self.read_ty.unwrap()
    }
}
