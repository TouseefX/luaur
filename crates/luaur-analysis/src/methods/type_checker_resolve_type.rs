//! @interface-stub
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use alloc::sync::Arc;
use luaur_ast::records::ast_type::AstType;

impl TypeChecker {
    pub fn resolve_type(&mut self, scope: ScopePtr, annotation: &AstType) -> TypeId {
        let ty = self.resolve_type_worker(scope, annotation);

        unsafe {
            let module = Arc::as_ptr(self.current_module.as_ref().unwrap())
                as *mut crate::records::module::Module;
            *(*module)
                .ast_resolved_types
                .get_or_insert(annotation as *const AstType) = ty;
        }

        ty
    }
}
