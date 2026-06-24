use crate::records::frontend::Frontend;
use crate::records::global_types::GlobalTypes;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use alloc::string::String;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Frontend {
    pub fn register_builtin_definition(
        &mut self,
        name: String,
        applicator: Box<dyn Fn(&mut Frontend, &mut GlobalTypes, ScopePtr)>,
    ) {
        LUAU_ASSERT!(!self.builtin_definitions.contains_key(&name));

        if !self.builtin_definitions.contains_key(&name) {
            self.builtin_definitions.insert(name, applicator.into());
        }
    }
}
