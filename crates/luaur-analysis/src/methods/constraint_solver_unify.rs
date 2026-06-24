//! `template<typename TID>
//!  bool ConstraintSolver::unify(NotNull<const Constraint> constraint, TID subTy, TID superTy)`
//! (`Analysis/src/ConstraintSolver.cpp:3833-3871`, hand-ported faithfully).

use core::ptr::NonNull;

use crate::enums::unify_result::UnifyResult;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::occurs_check_failed::OccursCheckFailed;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::subtyping_unifier::SubtypingUnifier;
use crate::records::unification_too_complex::UnificationTooComplex;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

/// Models the C++ `static_assert(std::is_same_v<TID, TypeId> || std::is_same_v<TID, TypePackId>)`
/// plus the `if constexpr` split over the `Subtyping::isSubtype` overload.
pub trait UnifyTid: Copy {
    fn is_subtype_in(
        self,
        subtyping: &mut Subtyping,
        super_ty: Self,
        scope: *mut Scope,
    ) -> SubtypingResult;
}

impl UnifyTid for TypeId {
    fn is_subtype_in(
        self,
        subtyping: &mut Subtyping,
        super_ty: Self,
        scope: *mut Scope,
    ) -> SubtypingResult {
        subtyping.is_subtype_type_id_type_id_not_null_scope(self, super_ty, scope)
    }
}

impl UnifyTid for TypePackId {
    fn is_subtype_in(
        self,
        subtyping: &mut Subtyping,
        super_ty: Self,
        scope: *mut Scope,
    ) -> SubtypingResult {
        subtyping.is_subtype_type_pack_id_type_pack_id_not_null_scope_vector_type_id(
            self,
            super_ty,
            scope,
            &alloc::vec::Vec::new(),
        )
    }
}

impl ConstraintSolver {
    pub fn constraint_solver_unify<TID: UnifyTid>(
        &mut self,
        constraint: *const Constraint,
        sub_ty: TID,
        super_ty: TID,
    ) -> bool {
        let scope = unsafe { (*constraint).scope };
        let location = unsafe { (*constraint).location };
        let ice_ptr =
            &self.ice_reporter as *const InternalErrorReporter as *mut InternalErrorReporter;

        let mut subtyping = Subtyping::subtyping_owned(
            self.builtin_types,
            self.arena,
            self.normalizer,
            self.type_function_runtime,
            ice_ptr,
        );
        let mut stu = SubtypingUnifier::subtyping_unifier(self.arena, self.builtin_types, ice_ptr);

        let mut result = sub_ty.is_subtype_in(&mut subtyping, super_ty, scope);

        let unifier_result =
            stu.dispatch_constraints(constraint, core::mem::take(&mut result.assumed_constraints));

        for cv in unifier_result.outstanding_constraints.iter() {
            let new_constraint =
                self.push_constraint(NonNull::new(scope).unwrap(), location, cv.clone());
            self.inherit_blocks(constraint, new_constraint.as_ptr());
        }

        for (ty, new_upper_bounds) in unifier_result.upper_bound_contributors.iter() {
            let upper_bounds = self.upper_bound_contributors.get_or_insert(*ty);
            upper_bounds.extend(new_upper_bounds.iter().cloned());
        }

        match unifier_result.unified {
            UnifyResult::OccursCheckFailed => {
                self.report_error_type_error_data_location(
                    OccursCheckFailed::default().into(),
                    &location,
                );
                false
            }
            UnifyResult::TooComplex => {
                self.report_error_type_error_data_location(
                    UnificationTooComplex::default().into(),
                    &location,
                );
                false
            }
            UnifyResult::Ok => true,
        }
    }
}
