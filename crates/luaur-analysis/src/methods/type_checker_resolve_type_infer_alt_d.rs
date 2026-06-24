use crate::functions::add_refinement::add_refinement;
use crate::records::truthy_predicate::TruthyPredicate;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::refinement_map::RefinementMap;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;

impl TypeChecker {
    pub fn resolve_truthy_predicate_refinement_map_scope_ptr_bool_bool(
        &mut self,
        truthy_p: &TruthyPredicate,
        refis: &mut RefinementMap,
        scope: ScopePtr,
        sense: bool,
        from_or: bool,
    ) {
        let ty = self.resolve_l_value_refinement_map_scope_ptr_l_value(
            refis,
            scope.clone(),
            &truthy_p.lvalue,
        );

        if let Some(ty) = ty {
            if from_or {
                add_refinement(refis, &truthy_p.lvalue, ty);
                return;
            }
        }

        let predicate = self.mk_truthy_predicate(sense, self.nil_type);
        self.refine_l_value(&truthy_p.lvalue, refis, scope, predicate);
    }
}
