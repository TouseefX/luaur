use crate::records::with_predicate::WithPredicate;
use crate::type_aliases::predicate_vec::PredicateVec;

impl<T: Default> WithPredicate<T> {
    pub fn with_predicate() -> Self {
        Self {
            r#type: T::default(),
            predicates: PredicateVec::default(),
        }
    }
}
