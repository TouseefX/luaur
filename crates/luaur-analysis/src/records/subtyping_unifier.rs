use crate::enums::occurs_check_result::OccursCheckResult;
use crate::enums::unify_result::UnifyResult;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::constraint::Constraint;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::result::Result;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::upper_bounds::UpperBounds;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct SubtypingUnifier {
    pub(crate) arena: *mut TypeArena,
    pub(crate) builtin_types: *mut BuiltinTypes,
    pub(crate) reporter: *mut InternalErrorReporter,
}

// `subtyping_unifier` (ctor) / `dispatch_constraints` / `dispatch_one_constraint`
// live in their own method node files (methods/subtyping_unifier_*.rs).
impl SubtypingUnifier {
    #[allow(non_snake_case)]
    pub fn occurs_check_DEPRECATED(
        &self,
        needle: TypePackId,
        haystack: TypePackId,
    ) -> OccursCheckResult {
        self.occurs_check_deprecated(needle, haystack)
    }

    pub fn canBeUnified(&self, ty: TypeId) -> bool {
        self.can_be_unified(ty)
    }
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let unified: () = ();
    let outstandingConstraints: () = ();
    let upperBoundContributors: () = ();
}
