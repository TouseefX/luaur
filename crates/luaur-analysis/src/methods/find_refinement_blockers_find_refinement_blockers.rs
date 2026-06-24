use crate::records::find_refinement_blockers::FindRefinementBlockers;
use crate::records::type_once_visitor::TypeOnceVisitor;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl FindRefinementBlockers {
    pub fn find_refinement_blockers() -> Self {
        FindRefinementBlockers {
            base: TypeOnceVisitor::new("FindRefinementBlockers".to_string(), true),
            found: DenseHashSet::new(core::ptr::null_mut()),
        }
    }
}
