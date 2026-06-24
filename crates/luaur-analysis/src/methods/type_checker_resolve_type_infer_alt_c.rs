use crate::functions::get_predicate::get_predicate;
use crate::records::and_predicate::AndPredicate;
use crate::records::eq_predicate::EqPredicate;
use crate::records::is_a_predicate::IsAPredicate;
use crate::records::not_predicate::NotPredicate;
use crate::records::or_predicate::OrPredicate;
use crate::records::truthy_predicate::TruthyPredicate;
use crate::records::type_checker::TypeChecker;
use crate::records::type_guard_predicate::TypeGuardPredicate;
use crate::type_aliases::predicate::Predicate;
use crate::type_aliases::refinement_map::RefinementMap;
use crate::type_aliases::scope_ptr_type::ScopePtr;

impl TypeChecker {
    pub fn resolve_predicate_refinement_map_scope_ptr_bool_bool(
        &mut self,
        predicate: &Predicate,
        refis: &mut RefinementMap,
        scope: &ScopePtr,
        sense: bool,
        from_or: bool,
    ) {
        if let Some(truthy_p) = unsafe { get_predicate::<TruthyPredicate>(predicate).as_ref() } {
            self.resolve_truthy_predicate_refinement_map_scope_ptr_bool_bool(
                truthy_p,
                refis,
                scope.clone(),
                sense,
                from_or,
            );
        } else if let Some(and_p) = unsafe { get_predicate::<AndPredicate>(predicate).as_ref() } {
            self.resolve_and_predicate_refinement_map_scope_ptr_bool(and_p, refis, scope, sense);
        } else if let Some(or_p) = unsafe { get_predicate::<OrPredicate>(predicate).as_ref() } {
            self.resolve_or_predicate_refinement_map_scope_ptr_bool(or_p, refis, scope, sense);
        } else if let Some(not_p) = unsafe { get_predicate::<NotPredicate>(predicate).as_ref() } {
            self.resolve_predicate_vec_refinement_map_scope_ptr_bool_bool(
                &not_p.predicates,
                refis,
                scope,
                !sense,
                from_or,
            );
        } else if let Some(isa_p) = unsafe { get_predicate::<IsAPredicate>(predicate).as_ref() } {
            self.resolve_is_a_predicate_refinement_map_scope_ptr_bool(
                isa_p,
                refis,
                scope.clone(),
                sense,
            );
        } else if let Some(typeguard_p) =
            unsafe { get_predicate::<TypeGuardPredicate>(predicate).as_ref() }
        {
            self.resolve_type_guard_predicate_refinement_map_scope_ptr_bool(
                typeguard_p,
                refis,
                scope.clone(),
                sense,
            );
        } else if let Some(eq_p) = unsafe { get_predicate::<EqPredicate>(predicate).as_ref() } {
            self.resolve_eq_predicate_refinement_map_scope_ptr_bool(
                eq_p,
                refis,
                scope.clone(),
                sense,
            );
        } else {
            self.ice_string("Unhandled predicate kind");
        }
    }
}
