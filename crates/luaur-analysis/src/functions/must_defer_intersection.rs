use crate::records::find_simplification_blockers::FindSimplificationBlockers;
use crate::records::iterative_type_visitor::IterativeTypeVisitorTrait;
use crate::type_aliases::type_id::TypeId;

pub fn must_defer_intersection(ty: TypeId) -> bool {
    let mut bts = FindSimplificationBlockers {
        base: Default::default(),
        found: false,
    };
    bts.find_simplification_blockers_find_simplification_blockers();
    bts.run_type_id(ty);
    bts.found
}
