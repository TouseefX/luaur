//! `bool ConstraintSolver::tryDispatch(const ReduceConstraint& c, NotNull<const Constraint> constraint, bool force)`
//! (`Analysis/src/ConstraintSolver.cpp:2879-2933`, hand-ported faithfully).

use core::ffi::c_void;
use core::ptr::NonNull;

use crate::functions::follow_type::follow_type_id;
use crate::functions::get_error::get_type_error;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::reduce_type_functions_type_function::reduce_type_functions;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::reduce_constraint::ReduceConstraint;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::uninhabited_type_function::UninhabitedTypeFunction;
use crate::records::uninhabited_type_pack_function::UninhabitedTypePackFunction;

impl ConstraintSolver {
    pub fn try_dispatch_reduce_constraint_not_null_constraint_bool(
        &mut self,
        c: &ReduceConstraint,
        constraint: *const Constraint,
        force: bool,
    ) -> bool {
        let mut ty = unsafe { follow_type_id(c.ty) };

        let scope = unsafe { (*constraint).scope };
        let location = unsafe { (*constraint).location };

        let mut context = TypeFunctionContext::from_solver(
            NonNull::new(self as *mut ConstraintSolver).unwrap(),
            NonNull::new(scope).unwrap(),
            NonNull::new(constraint as *mut Constraint).unwrap(),
            NonNull::new(self.subtyping).unwrap(),
        );
        let mut result = reduce_type_functions(
            ty,
            location,
            NonNull::new(&mut context as *mut TypeFunctionContext).unwrap(),
            force,
        );

        for r in result.reduced_types.iter() {
            self.unblock_type_id_location(*r, location);
        }

        for r in result.reduced_packs.iter() {
            self.unblock_type_pack_id_location(*r, location);
        }

        for ity in result.irreducible_types.iter() {
            self.uninhabited_type_functions
                .insert(*ity as *const c_void);
            self.unblock_type_id_location(*ity, location);
        }

        let reduction_finished = result.blocked_types.empty() && result.blocked_packs.empty();

        ty = unsafe { follow_type_id(ty) };

        // If we couldn't reduce this type function, stick it in the set!
        if unsafe { !get_type_id::<TypeFunctionInstanceType>(ty).is_null() }
            && result.irreducible_types.find(&ty).is_none()
        {
            *self.type_functions_to_finalize.get_or_insert(ty) = constraint;
        }

        if force || reduction_finished {
            for message in core::mem::take(&mut result.messages) {
                self.report_error_type_error(message);
            }

            // if we're completely dispatching this constraint, we want to record any uninhabited type functions to unblock.
            for error in result.errors.iter() {
                let utf = get_type_error::<UninhabitedTypeFunction>(error);
                if !utf.is_null() {
                    self.uninhabited_type_functions
                        .insert(unsafe { (*utf).ty } as *const c_void);
                } else {
                    let utpf = get_type_error::<UninhabitedTypePackFunction>(error);
                    if !utpf.is_null() {
                        self.uninhabited_type_functions
                            .insert(unsafe { (*utpf).tp } as *const c_void);
                    }
                }
            }
        }

        if force {
            return true;
        }

        for b in result.blocked_types.iter() {
            self.block_type_id_not_null_constraint(*b, constraint);
        }

        for b in result.blocked_packs.iter() {
            self.block_type_pack_id_not_null_constraint(*b, constraint);
        }

        reduction_finished
    }
}
