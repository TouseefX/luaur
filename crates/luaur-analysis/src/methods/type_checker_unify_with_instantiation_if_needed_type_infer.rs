use crate::records::type_checker::TypeChecker;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl TypeChecker {
    pub fn unify_with_instantiation_if_needed_type_id_type_id_scope_ptr_location(
        &mut self,
        sub_ty: TypeId,
        super_ty: TypeId,
        scope: ScopePtr,
        location: &Location,
    ) -> bool {
        let mut state = self.mk_unifier(&scope, location);
        self.unify_with_instantiation_if_needed_type_id_type_id_scope_ptr_unifier(
            sub_ty, super_ty, scope, &mut state,
        );

        state.log.commit();
        self.report_errors(&state.errors);

        state.errors.is_empty()
    }
}
