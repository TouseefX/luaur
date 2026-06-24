use crate::records::frontend::Frontend;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use alloc::string::String;

impl Frontend {
    pub fn add_environment(&mut self, environment_name: String) -> ScopePtr {
        if let Some(scope) = self.environments.get(&environment_name) {
            return scope.clone();
        }
        let scope = ScopePtr::new(crate::records::scope::Scope::new(
            &self.globals.global_scope,
            0,
        ));
        self.environments.insert(environment_name, scope.clone());
        scope
    }
}
