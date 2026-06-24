use crate::records::type_checker::TypeChecker;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl TypeChecker {
    pub fn try_unify(
        &mut self,
        sub_ty: TypeId,
        super_ty: TypeId,
        scope: &ScopePtr,
        location: &Location,
    ) -> ErrorVec {
        // `mk_unifier` resets the shared iteration counter (the per-top-level-unify
        // reset C++ does in the public `Unifier::tryUnify`), so this calls the
        // recursive `tryUnify_` directly, as C++ `tryUnify` does after the reset.
        let mut state = self.mk_unifier(scope, location);

        state.try_unify_type_id_type_id_bool_bool_literal_properties(
            sub_ty, super_ty, false, false, None,
        );

        if state.errors.is_empty() {
            state.log.commit();
        }

        state.errors
    }
}
