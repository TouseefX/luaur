use crate::records::type_checker::TypeChecker;
use crate::type_aliases::l_value::LValue;
use crate::type_aliases::refinement_map::RefinementMap;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;

impl TypeChecker {
    pub fn resolve_l_value_refinement_map_scope_ptr_l_value(
        &mut self,
        refis: &RefinementMap,
        scope: ScopePtr,
        lvalue: &LValue,
    ) -> Option<TypeId> {
        if let Some(ty) = refis.get(lvalue) {
            Some(*ty)
        } else {
            self.resolve_l_value_scope_ptr_l_value(scope, lvalue)
        }
    }
}
