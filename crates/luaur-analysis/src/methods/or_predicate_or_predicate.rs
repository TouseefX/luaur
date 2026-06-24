//! @interface-stub
use crate::records::or_predicate::OrPredicate;
use crate::type_aliases::predicate_vec::PredicateVec;

impl OrPredicate {
    pub fn new(lhs: PredicateVec, rhs: PredicateVec) -> Self {
        OrPredicate { lhs, rhs }
    }
}
