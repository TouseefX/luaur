use crate::records::frontend::Frontend;
use crate::records::global_types::GlobalTypes;
use alloc::string::String;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Frontend {
    pub fn apply_builtin_definition_to_environment(
        &mut self,
        environment_name: String,
        definition_name: String,
    ) {
        LUAU_ASSERT!(self.builtin_definitions.contains_key(&definition_name));

        if self.builtin_definitions.contains_key(&definition_name) {
            let applicator = self
                .builtin_definitions
                .get(&definition_name)
                .unwrap()
                .clone();
            let environment_scope = self.get_environment_scope(environment_name);
            // C++: builtinDefinitions[definitionName](*this, globals, getEnvironmentScope(...));
            // The C++ passes both *this and the `globals` member by mutable reference at the
            // same call; Rust forbids aliasing &mut self with &mut self.globals, so split the
            // borrow through a raw pointer to the `globals` field (the applicator never
            // re-enters `self.globals` through `self`, matching C++ semantics).
            let globals_ptr: *mut GlobalTypes = &mut self.globals;
            applicator(self, unsafe { &mut *globals_ptr }, environment_scope);
        }
    }
}
