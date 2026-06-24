//! Faithful port of `TypeChecker2::findInnermostScope` (TypeChecker2.cpp:609-629).
use crate::records::scope::Scope;
use crate::records::type_checker_2::TypeChecker2;
use alloc::sync::Arc;
use luaur_ast::records::location::Location;

impl TypeChecker2 {
    pub fn find_innermost_scope(&self, location: Location) -> *mut Scope {
        // Scope* bestScope = module->getModuleScope().get();
        let module_scope = unsafe { (*self.module).get_module_scope() };
        let mut best_scope = Arc::as_ptr(&module_scope) as *mut Scope;

        let mut did_narrow;
        loop {
            did_narrow = false;
            for &scope in unsafe { &(*best_scope).children } {
                if unsafe { (*scope).location.encloses(&location) } {
                    best_scope = scope;
                    did_narrow = true;
                    break;
                }
            }

            if !(did_narrow && unsafe { !(*best_scope).children.is_empty() }) {
                break;
            }
        }

        best_scope
    }
}
