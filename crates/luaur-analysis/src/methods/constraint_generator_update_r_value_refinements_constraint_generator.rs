use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::scope::Scope;
use crate::type_aliases::def_id_def::DefId;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use crate::type_aliases::type_id::TypeId;

impl ConstraintGenerator {
    pub fn update_r_value_refinements_scope_ptr_def_id_type_id(
        &self,
        scope: &ScopePtr,
        def: DefId,
        ty: TypeId,
    ) {
        let scope_raw = scope.as_ref() as *const Scope as *mut Scope;
        self.update_r_value_refinements_scope_def_id_type_id(scope_raw, def, ty);
    }
}
