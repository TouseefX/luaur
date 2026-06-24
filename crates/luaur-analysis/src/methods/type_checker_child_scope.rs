//! @interface-stub
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use alloc::sync::Arc;
use luaur_ast::records::location::Location;

impl TypeChecker {
    pub fn child_scope(&mut self, parent: &ScopePtr, location: &Location) -> ScopePtr {
        let mut scope_value = crate::records::scope::Scope::new(parent, 0);
        scope_value.level = parent.level;
        scope_value.vararg_pack = parent.vararg_pack;
        scope_value.location = *location;
        scope_value.return_type = parent.return_type;

        let scope = Arc::new(scope_value);

        unsafe {
            let parent_mut = Arc::as_ptr(parent) as *mut crate::records::scope::Scope;
            (*parent_mut)
                .children
                .push(Arc::as_ptr(&scope) as *mut crate::records::scope::Scope);

            let module = Arc::as_ptr(self.current_module.as_ref().unwrap())
                as *mut crate::records::module::Module;
            (*module).scopes.push((*location, scope.clone()));
        }

        scope
    }
}
