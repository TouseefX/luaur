use crate::records::fixture::Fixture;
use alloc::string::String;
use luaur_analysis::type_aliases::type_id::TypeId;
use luaur_common::LUAU_ASSERT;

use luaur_analysis::functions::follow_type::follow_type_id;
use luaur_analysis::functions::follow_type_utils::follow_optional_ty;

impl Fixture {
    pub fn require_type_string(&mut self, name: &String) -> TypeId {
        let ty = self.get_type(name, false);
        LUAU_ASSERT!(ty.is_some());
        unsafe { follow_optional_ty(ty) }.unwrap_or(core::ptr::null_mut())
    }
}
