use crate::records::type_checker::TypeChecker;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use alloc::sync::Arc;
use luaur_ast::records::location::Location;

impl TypeChecker {
    pub fn child_function_scope(
        &mut self,
        parent: &ScopePtr,
        location: &Location,
        sub_level: i32,
    ) -> ScopePtr {
        let mut scope_value = crate::records::scope::Scope::new(parent, sub_level);
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
