use crate::records::type_checker::TypeChecker;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl TypeChecker {
    pub fn can_unify_type_id_type_id_scope_ptr_location(
        &mut self,
        sub_ty: TypeId,
        super_ty: TypeId,
        scope: &ScopePtr,
        location: &Location,
    ) -> ErrorVec {
        self.can_unify_type_infer(sub_ty, super_ty, scope, location)
    }
}
