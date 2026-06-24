//! `NotNull<Constraint> ConstraintSolver::pushConstraint(NotNull<Scope> scope, const Location& location, ConstraintV cv)`
//! (`Analysis/src/ConstraintSolver.cpp:4094-4126`, hand-ported faithfully).

use alloc::boxed::Box;
use core::ptr::NonNull;

use crate::enums::subtyping_variance::SubtypingVariance;
use crate::records::code_too_complex::CodeTooComplex;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::equality_constraint::EqualityConstraint;
use crate::records::hash_subtype_constraint_record::HashSubtypeConstraintRecord;
use crate::records::scope::Scope;
use crate::records::subtype_constraint::SubtypeConstraint;
use crate::records::subtype_constraint_record::SubtypeConstraintRecord;
use crate::type_aliases::constraint_v::{ConstraintV, ConstraintVMember};
use luaur_ast::records::location::Location;
use luaur_common::records::dense_hash_table::DenseHasher;

// C++ `size_t HashSubtypeConstraintRecord::operator()(const SubtypeConstraintRecord&)`
// wired into the `DenseHashMap` hasher functor slot (the `operator()` body is
// ported in `methods/hash_subtype_constraint_record_operator_call.rs`).
impl DenseHasher<SubtypeConstraintRecord> for HashSubtypeConstraintRecord {
    fn hash(&self, key: &SubtypeConstraintRecord) -> usize {
        self.operator_call(key)
    }
}

impl ConstraintSolver {
    pub fn push_constraint(
        &mut self,
        scope: NonNull<Scope>,
        location: Location,
        cv: ConstraintV,
    ) -> NonNull<Constraint> {
        let mut scr: Option<SubtypeConstraintRecord> = None;
        if let Some(sc) = SubtypeConstraint::get_if(&cv) {
            scr = Some(SubtypeConstraintRecord {
                subTy: sc.sub_type,
                superTy: sc.super_type,
                variance: SubtypingVariance::Covariant,
            });
        } else if let Some(ec) = EqualityConstraint::get_if(&cv) {
            scr = Some(SubtypeConstraintRecord {
                subTy: ec.assignment_type,
                superTy: ec.result_type,
                variance: SubtypingVariance::Invariant,
            });
        }

        if let Some(record) = scr {
            if let Some(f) = self.seen_constraints.find(&record) {
                return NonNull::new(*f).unwrap();
            }
        }

        let c: Box<Constraint> = Box::new(
            Constraint::constraint_not_null_scope_location_constraint_v(scope, &location, cv),
        );
        let borrow: *mut Constraint = c.as_ref() as *const Constraint as *mut Constraint;

        if let Some(record) = scr {
            *self.seen_constraints.get_or_insert(record) = borrow;
        }

        self.solver_constraints.push(c);
        self.unsolved_constraints.push(borrow as *const Constraint);

        if self.solver_constraint_limit > 0 {
            self.solver_constraint_limit -= 1;

            if self.solver_constraint_limit == 0 {
                self.report_error_type_error_data_location(
                    CodeTooComplex::default().into(),
                    &location,
                );
            }
        }

        NonNull::new(borrow).unwrap()
    }
}
