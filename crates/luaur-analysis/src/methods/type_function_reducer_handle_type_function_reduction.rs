//! `TypeFunctionReducer::handleTypeFunctionReduction<T>` (TypeFunction.cpp:375-453).
//!
//! C++ is a template branching on `std::is_same_v<T, TypeId|TypePackId>`. It is
//! rendered as the two concrete `handle_type_function_reduction_type_id` /
//! `..._type_pack_id` methods. The reduction result struct in this crate is
//! monomorphized on `TypeId`.

use crate::enums::reduction::Reduction;
use crate::enums::type_function_instance_state::TypeFunctionInstanceState;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::reduce_constraint::ReduceConstraint;
use crate::records::type_error::TypeError;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_reducer::TypeFunctionReducer;
use crate::records::type_function_reduction_result::TypeFunctionReductionResult;
use crate::records::uninhabited_type_function::UninhabitedTypeFunction;
use crate::records::uninhabited_type_pack_function::UninhabitedTypePackFunction;
use crate::records::user_defined_type_function_error::UserDefinedTypeFunctionError;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeFunctionReducer {
    pub fn handle_type_function_reduction_type_id(
        &mut self,
        subject: TypeId,
        mut reduction: TypeFunctionReductionResult,
    ) {
        for message in core::mem::take(&mut reduction.messages) {
            self.result
                .messages
                .push(TypeError::type_error_location_type_error_data(
                    self.location,
                    TypeErrorData::UserDefinedTypeFunctionError(UserDefinedTypeFunctionError::new(
                        message,
                    )),
                ));
        }

        if let Some(result_ty) = reduction.result {
            self.replace_type_id(subject, result_ty);

            // Collect fresh instances first so we are not holding the borrow on
            // ctx across the mutable push onto our own queue.
            let fresh: alloc::vec::Vec<TypeId> =
                unsafe { (*self.ctx.as_ptr()).fresh_instances.clone() };
            let has_solver = unsafe { !(*self.ctx.as_ptr()).solver.is_null() };
            for ty in fresh {
                self.queued_tys.push_back(ty);
                if has_solver {
                    // C++: `ctx->pushConstraint(ReduceConstraint{ty})`.
                    unsafe {
                        (*self.ctx.as_ptr())
                            .push_constraint(ConstraintV::Reduce(ReduceConstraint { ty }));
                    }
                }
            }
        } else {
            self.irreducible.insert(subject as *const core::ffi::c_void);

            if let Some(error) = reduction.error.take() {
                self.result
                    .errors
                    .push(TypeError::type_error_location_type_error_data(
                        self.location,
                        TypeErrorData::UserDefinedTypeFunctionError(
                            UserDefinedTypeFunctionError::new(error),
                        ),
                    ));
            }

            if reduction.reduction_status != Reduction::MaybeOk || self.force {
                if self.get_state_type_id(subject) == TypeFunctionInstanceState::Unsolved {
                    if reduction.reduction_status == Reduction::Erroneous {
                        self.set_state_type_id_type_function_instance_state(
                            subject,
                            TypeFunctionInstanceState::Stuck,
                        );
                    } else if reduction.reduction_status == Reduction::Irreducible {
                        self.set_state_type_id_type_function_instance_state(
                            subject,
                            TypeFunctionInstanceState::Solved,
                        );
                    } else if reduction.reduction_status == Reduction::MaybeOk {
                        // We cannot make progress because something is unsolved, but we're also forcing.
                        self.set_state_type_id_type_function_instance_state(
                            subject,
                            TypeFunctionInstanceState::Stuck,
                        );
                    } else {
                        unsafe {
                            (*self.ctx.as_ptr())
                                .ice
                                .as_ref()
                                .ice_string("Unexpected TypeFunctionInstanceState");
                        }
                    }
                }

                let tf = unsafe { get_type_id::<TypeFunctionInstanceType>(subject) };
                if !tf.is_null() {
                    let is_user_func = unsafe {
                        core::ptr::eq(
                            (*tf).function.as_ptr() as *const _,
                            &(*self.ctx.as_ptr())
                                .builtins
                                .as_ref()
                                .typeFunctions
                                .user_func as *const _,
                        )
                    };
                    if !is_user_func {
                        self.result
                            .errors
                            .push(TypeError::type_error_location_type_error_data(
                                self.location,
                                TypeErrorData::UninhabitedTypeFunction(UninhabitedTypeFunction {
                                    ty: subject,
                                }),
                            ));
                    }
                }
            } else if reduction.reduction_status == Reduction::MaybeOk && !self.force {
                // We're not forcing and the reduction couldn't proceed, but it isn't obviously busted.
                // Report that this type blocks further reduction.
                for b in reduction.blocked_types.iter().copied() {
                    self.result.blocked_types.insert(b);
                }

                for b in reduction.blocked_packs.iter().copied() {
                    self.result.blocked_packs.insert(b);
                }
            } else {
                debug_assert!(false, "Unreachable");
            }
        }

        unsafe {
            (*self.ctx.as_ptr()).fresh_instances.clear();
        }
    }

    pub fn handle_type_function_reduction_type_pack_id(
        &mut self,
        subject: TypePackId,
        mut reduction: TypeFunctionReductionResult,
    ) {
        for message in core::mem::take(&mut reduction.messages) {
            self.result
                .messages
                .push(TypeError::type_error_location_type_error_data(
                    self.location,
                    TypeErrorData::UserDefinedTypeFunctionError(UserDefinedTypeFunctionError::new(
                        message,
                    )),
                ));
        }

        // NOTE: The reduction result struct is monomorphized on `TypeId` in this
        // crate, while a pack reduction would carry a `TypePackId`. There are no
        // type pack functions at present (see `TypeFunctionReducer::get_state`
        // for packs), so a pack reducer never produces a `result`; the success
        // arm (`replace`) is therefore unreachable and not rendered here.
        {
            self.irreducible.insert(subject as *const core::ffi::c_void);

            if let Some(error) = reduction.error.take() {
                self.result
                    .errors
                    .push(TypeError::type_error_location_type_error_data(
                        self.location,
                        TypeErrorData::UserDefinedTypeFunctionError(
                            UserDefinedTypeFunctionError::new(error),
                        ),
                    ));
            }

            if reduction.reduction_status != Reduction::MaybeOk || self.force {
                if self.get_state_type_pack_id(subject) == TypeFunctionInstanceState::Unsolved {
                    if reduction.reduction_status == Reduction::Erroneous {
                        self.set_state_type_pack_id_type_function_instance_state(
                            subject,
                            TypeFunctionInstanceState::Stuck,
                        );
                    } else if reduction.reduction_status == Reduction::Irreducible {
                        self.set_state_type_pack_id_type_function_instance_state(
                            subject,
                            TypeFunctionInstanceState::Solved,
                        );
                    } else if reduction.reduction_status == Reduction::MaybeOk {
                        self.set_state_type_pack_id_type_function_instance_state(
                            subject,
                            TypeFunctionInstanceState::Stuck,
                        );
                    } else {
                        unsafe {
                            (*self.ctx.as_ptr())
                                .ice
                                .as_ref()
                                .ice_string("Unexpected TypeFunctionInstanceState");
                        }
                    }
                }

                self.result
                    .errors
                    .push(TypeError::type_error_location_type_error_data(
                        self.location,
                        TypeErrorData::UninhabitedTypePackFunction(UninhabitedTypePackFunction {
                            tp: subject,
                        }),
                    ));
            } else if reduction.reduction_status == Reduction::MaybeOk && !self.force {
                for b in reduction.blocked_types.iter().copied() {
                    self.result.blocked_types.insert(b);
                }

                for b in reduction.blocked_packs.iter().copied() {
                    self.result.blocked_packs.insert(b);
                }
            } else {
                debug_assert!(false, "Unreachable");
            }
        }

        unsafe {
            (*self.ctx.as_ptr()).fresh_instances.clear();
        }
    }
}
