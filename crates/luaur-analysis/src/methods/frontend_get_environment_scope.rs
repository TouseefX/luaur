use crate::records::frontend::Frontend;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use alloc::string::String;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Frontend {
    pub fn get_environment_scope(&self, environment_name: String) -> ScopePtr {
        if let Some(scope) = self.environments.get(&environment_name) {
            return scope.clone();
        }

        LUAU_ASSERT!(false);
        unsafe { core::mem::zeroed() }
    }
}
