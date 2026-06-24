use crate::records::and_predicate::AndPredicate;
use crate::records::not_predicate::NotPredicate;
use crate::records::or_predicate::OrPredicate;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::predicate::Predicate;
use crate::type_aliases::refinement_map::RefinementMap;
use crate::type_aliases::scope_ptr_type::ScopePtr;

impl TypeChecker {
    pub fn resolve_and_predicate_refinement_map_scope_ptr_bool(
        &mut self,
        and_p: &AndPredicate,
        refis: &mut RefinementMap,
        scope: &ScopePtr,
        sense: bool,
    ) {
        if !sense {
            let or_p = OrPredicate {
                lhs: alloc::vec![Predicate::Not(NotPredicate {
                    predicates: and_p.lhs.clone(),
                })],
                rhs: alloc::vec![Predicate::Not(NotPredicate {
                    predicates: and_p.rhs.clone(),
                })],
            };

            self.resolve_or_predicate_refinement_map_scope_ptr_bool(&or_p, refis, scope, !sense);
            return;
        }

        self.resolve_predicate_vec_refinement_map_scope_ptr_bool_bool(
            &and_p.lhs, refis, scope, sense, false,
        );
        self.resolve_predicate_vec_refinement_map_scope_ptr_bool_bool(
            &and_p.rhs, refis, scope, sense, false,
        );
    }
}
