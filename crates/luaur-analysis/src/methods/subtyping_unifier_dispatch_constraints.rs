//! Source: `Analysis/src/SubtypingUnifier.cpp:33-52` — `SubtypingUnifier::dispatchConstraints`.

use crate::enums::unify_result::UnifyResult;
use crate::records::constraint::Constraint;
use crate::records::result::Result;
use crate::records::subtyping_unifier::SubtypingUnifier;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::upper_bounds::UpperBounds;
use alloc::vec::Vec;

impl SubtypingUnifier {
    pub fn dispatch_constraints(
        &self,
        constraint: *const Constraint,
        assumed_constraints: Vec<ConstraintV>,
    ) -> Result {
        let mut unifier_res = UnifyResult::Ok;
        // NOTE: You *could* potentially reuse the input vector, but this seems
        // easier to read.
        let mut outstanding_constraints: Vec<ConstraintV> = Vec::new();
        outstanding_constraints.reserve(assumed_constraints.len());
        let mut upper_bounds: UpperBounds = UpperBounds::new(core::ptr::null());
        for cv in assumed_constraints {
            let (unified, dispatched) =
                self.dispatch_one_constraint(constraint, &cv, &mut upper_bounds);
            unifier_res &= unified;
            if !dispatched {
                outstanding_constraints.push(cv);
            }
        }
        Result {
            unified: unifier_res,
            outstanding_constraints,
            upper_bound_contributors: upper_bounds,
        }
    }
}
