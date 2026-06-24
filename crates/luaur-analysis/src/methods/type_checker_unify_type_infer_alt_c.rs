use crate::records::count_mismatch::CountMismatchContext;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::location::Location;

impl TypeChecker {
    pub fn unify_type_pack_id_type_pack_id_scope_ptr_location_count_mismatch_context(
        &mut self,
        sub_ty: TypePackId,
        super_ty: TypePackId,
        scope: &ScopePtr,
        location: &Location,
        ctx: CountMismatchContext,
    ) -> bool {
        let mut state = self.mk_unifier(scope, location);
        state.ctx = ctx;
        state.try_unify_type_pack_id_type_pack_id_bool(sub_ty, super_ty, false);

        state.log.commit();

        self.report_errors(&state.errors);

        state.errors.is_empty()
    }
}
