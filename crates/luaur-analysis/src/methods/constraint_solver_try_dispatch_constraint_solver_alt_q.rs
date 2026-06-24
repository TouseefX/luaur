//! `bool ConstraintSolver::tryDispatch(const ReducePackConstraint& c, NotNull<const Constraint> constraint, bool force)`
//! (`Analysis/src/ConstraintSolver.cpp:2935-2972`, hand-ported faithfully).

use core::ffi::c_void;
use core::ptr::NonNull;

use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_error::get_type_error;
use crate::functions::reduce_type_functions_type_function_alt_b::reduce_type_functions;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::reduce_pack_constraint::ReducePackConstraint;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::uninhabited_type_function::UninhabitedTypeFunction;
use crate::records::uninhabited_type_pack_function::UninhabitedTypePackFunction;

impl ConstraintSolver {
    pub fn try_dispatch_reduce_pack_constraint_not_null_constraint_bool(
        &mut self,
        c: &ReducePackConstraint,
        constraint: *const Constraint,
        force: bool,
    ) -> bool {
        let tp = unsafe { follow_type_pack_id(c.tp) };

        let scope = unsafe { (*constraint).scope };
        let location = unsafe { (*constraint).location };

        let mut context = TypeFunctionContext::from_solver(
            NonNull::new(self as *mut ConstraintSolver).unwrap(),
            NonNull::new(scope).unwrap(),
            NonNull::new(constraint as *mut Constraint).unwrap(),
            NonNull::new(self.subtyping).unwrap(),
        );
        let result = reduce_type_functions(
            tp,
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

        let reduction_finished = result.blocked_types.empty() && result.blocked_packs.empty();

        if force || reduction_finished {
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
