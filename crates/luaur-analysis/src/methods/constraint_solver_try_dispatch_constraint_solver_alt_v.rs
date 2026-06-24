use crate::functions::push_type_into::push_type_into;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::push_type_constraint::PushTypeConstraint;
use crate::records::subtyping::Subtyping;
use crate::records::unifier_2::Unifier2;
use core::ffi::c_void;
use core::ptr::NonNull;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl ConstraintSolver {
    pub fn try_dispatch_push_type_constraint_not_null_constraint_bool(
        &mut self,
        c: &PushTypeConstraint,
        constraint: *const Constraint,
        force: bool,
    ) -> bool {
        let mut u2 = Unifier2::unifier_2_not_null_type_arena_not_null_builtin_types_not_null_scope_not_null_internal_error_reporter_dense_hash_set_void(
            NonNull::new(self.arena).unwrap(),
            NonNull::new(self.builtin_types).unwrap(),
            NonNull::new(unsafe { (*constraint).scope }).unwrap(),
            NonNull::new(&self.ice_reporter as *const InternalErrorReporter as *mut InternalErrorReporter).unwrap(),
            &mut self.uninhabited_type_functions as *mut DenseHashSet<*const c_void>,
        );

        let mut subtyping = Subtyping::subtyping_owned(
            self.builtin_types,
            self.arena,
            self.normalizer,
            self.type_function_runtime,
            &self.ice_reporter as *const InternalErrorReporter as *mut InternalErrorReporter,
        );

        // NOTE: If we don't do this check up front, we almost immediately start
        // spawning tons of push type constraints. It's pretty important.
        if self.is_blocked_type_id(c.expectedType) {
            self.block_type_id_not_null_constraint(c.expectedType, constraint);
            // If we're forcing this constraint and the expected type is blocked, we
            // should just bail.
            return force;
        }

        let mut empty: DenseHashSet<*const c_void> = DenseHashSet::new(core::ptr::null_mut());
        let result = push_type_into(
            NonNull::new(
                c.astTypes
                    as *mut DenseHashMap<*const AstExpr, crate::type_aliases::type_id::TypeId>,
            )
            .unwrap(),
            NonNull::new(
                c.astExpectedTypes
                    as *mut DenseHashMap<*const AstExpr, crate::type_aliases::type_id::TypeId>,
            )
            .unwrap(),
            NonNull::new(self as *mut ConstraintSolver).unwrap(),
            NonNull::new(constraint as *mut Constraint).unwrap(),
            NonNull::new(&mut empty as *mut DenseHashSet<*const c_void>).unwrap(),
            NonNull::new(&mut u2 as *mut Unifier2).unwrap(),
            NonNull::new(&mut subtyping as *mut Subtyping).unwrap(),
            c.expectedType,
            c.expr,
        );

        // If we're forcing this constraint, just early exit: we can continue
        // inferring the rest of the file, we might just error when we shouldn't.
        if force || result.incomplete_types.is_empty() {
            return true;
        }

        for incomplete in &result.incomplete_types {
            let addition = self.push_constraint(
                NonNull::new(unsafe { (*constraint).scope }).unwrap(),
                unsafe { (*constraint).location },
                crate::type_aliases::constraint_v::ConstraintV::PushType(PushTypeConstraint {
                    expectedType: incomplete.expectedType,
                    targetType: incomplete.targetType,
                    astTypes: c.astTypes,
                    astExpectedTypes: c.astExpectedTypes,
                    expr: incomplete.expr,
                }),
            );
            self.inherit_blocks(constraint, addition.as_ptr());
        }

        true
    }
}
