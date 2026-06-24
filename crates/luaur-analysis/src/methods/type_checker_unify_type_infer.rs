use crate::records::type_checker::TypeChecker;
use crate::records::unifier_options::UnifierOptions;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl TypeChecker {
    pub fn unify_type_id_type_id_scope_ptr_location(
        &mut self,
        sub_ty: TypeId,
        super_ty: TypeId,
        scope: &ScopePtr,
        location: &Location,
    ) -> bool {
        let options = UnifierOptions::default();
        self.unify_type_id_type_id_scope_ptr_location_unifier_options(
            sub_ty, super_ty, scope, location, &options,
        )
    }
}
