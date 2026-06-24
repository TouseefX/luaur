use crate::functions::begin_type_pack::begin_type_pack_id;
use crate::functions::end_type_pack::end_type_pack_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::contains_any_generic_deprecated::ContainsAnyGenericDeprecated;
use crate::records::free_type::FreeType;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::function_type::FunctionType;
use crate::records::push_function_type_constraint::PushFunctionTypeConstraint;
use luaur_common::FFlag;

impl ConstraintSolver {
    pub fn try_dispatch_push_function_type_constraint_not_null_constraint(
        &mut self,
        c: &PushFunctionTypeConstraint,
        constraint: *const Constraint,
    ) -> bool {
        let expected_fn = unsafe {
            let followed = follow_type_id(c.expected_function_type);
            get_type_id::<FunctionType>(followed)
        };

        let fn_ty = unsafe {
            let followed = follow_type_id(c.function_type);
            get_type_id::<FunctionType>(followed)
        };

        if expected_fn.is_null() || fn_ty.is_null() {
            return true;
        }

        let expected_fn_ref = unsafe { &*expected_fn };
        let fn_ty_ref = unsafe { &*fn_ty };

        if FFlag::LuauInstantiateFunctionTypeBeforePush.get() {
            // instantiate is not yet translated; stubbing the logic as per the source comment
            // "NOTE: This logic could probably be combined with that of FunctionCheckConstraint"
            // Since instantiate is not available, we skip this block and rely on the fallback behavior
        }

        let mut expected_params = unsafe { begin_type_pack_id(expected_fn_ref.arg_types) };
        let mut params = unsafe { begin_type_pack_id(fn_ty_ref.arg_types) };

        let expected_params_end = unsafe { end_type_pack_id(expected_fn_ref.arg_types) };
        let params_end = unsafe { end_type_pack_id(fn_ty_ref.arg_types) };

        if expected_params.operator_eq(&expected_params_end) || params.operator_eq(&params_end) {
            return true;
        }

        if c.is_self {
            let params_current = unsafe { *params.operator_deref() };
            if !unsafe { get_type_id::<FreeType>(follow_type_id(params_current)) }.is_null() {
                if !FFlag::LuauConstraintGraph.get() {
                    self.deprecate_d_shift_references(params_current, unsafe {
                        *expected_params.operator_deref()
                    });
                }
                self.bind_not_null_constraint_type_id_type_id(constraint, params_current, unsafe {
                    *expected_params.operator_deref()
                });
            }
            expected_params.operator_inc();
            params.operator_inc();
        }

        let mut idx: usize = 0;
        while idx < unsafe { (*c.expr).args.size }
            && !expected_params.operator_eq(&expected_params_end)
            && !params.operator_eq(&params_end)
        {
            let arg = unsafe { *(*c.expr).args.data.add(idx) };
            let annotation = unsafe { (*arg).annotation };
            let params_current = unsafe { *params.operator_deref() };
            let free_type_ptr = unsafe { get_type_id::<FreeType>(follow_type_id(params_current)) };

            if annotation.is_null()
                && !free_type_ptr.is_null()
                && (FFlag::LuauInstantiateFunctionTypeBeforePush.get()
                    || !ContainsAnyGenericDeprecated::has_any_generic(unsafe {
                        *expected_params.operator_deref()
                    }))
            {
                if !FFlag::LuauConstraintGraph.get() {
                    self.deprecate_d_shift_references(params_current, unsafe {
                        *expected_params.operator_deref()
                    });
                }
                self.bind_not_null_constraint_type_id_type_id(constraint, params_current, unsafe {
                    *expected_params.operator_deref()
                });
            }

            expected_params.operator_inc();
            params.operator_inc();
            idx += 1;
        }

        if unsafe { (*c.expr).return_annotation.is_null() }
            && !unsafe {
                crate::functions::get_type_pack::get_type_pack_id::<FreeTypePack>(
                    fn_ty_ref.ret_types,
                )
            }
            .is_null()
            && (FFlag::LuauInstantiateFunctionTypeBeforePush.get()
                || !ContainsAnyGenericDeprecated::has_any_generic_type_pack_id(
                    expected_fn_ref.ret_types,
                ))
        {
            self.bind_not_null_constraint_type_pack_id_type_pack_id(
                constraint,
                fn_ty_ref.ret_types,
                expected_fn_ref.ret_types,
            );
        }

        true
    }
}
