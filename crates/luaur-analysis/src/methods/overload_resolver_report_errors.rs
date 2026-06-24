//! Source: `Analysis/src/OverloadResolver.cpp:289-453` (hand-ported)
use crate::enums::subtyping_variance::SubtypingVariance;
use crate::enums::value::Value;
use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::get_argument_index::get_argument_index;
use crate::functions::get_parameter_extents::get_parameter_extents;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::functions::is_path_on_argument_list::is_path_on_argument_list;
use crate::functions::is_variadic_type_pack::is_variadic;
use crate::functions::should_suppress_errors_type_utils_alt_b::should_suppress_errors_not_null_normalizer_type_pack_id;
use crate::functions::traverse_for_flattened_pack::traverse_for_flattened_pack;
use crate::functions::traverse_for_pack_type_path::traverse_for_pack;
use crate::functions::traverse_type_path_alt_b::traverse as traverse_pack_root;
use crate::functions::traverse_type_path_alt_c::traverse as traverse_type_root;
use crate::records::count_mismatch::{CountMismatch, CountMismatchContext};
use crate::records::function_type::FunctionType;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::internal_error::InternalError;
use crate::records::normalization_too_complex::NormalizationTooComplex;
use crate::records::overload_resolver::OverloadResolver;
use crate::records::subtyping_reasoning::SubtypingReasoning;
use crate::records::txn_log::TxnLog;
use crate::records::type_error::TypeError;
use crate::records::type_pack_mismatch::TypePackMismatch;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::module_name_type_fwd::ModuleName;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::location::Location;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl OverloadResolver {
    pub fn report_errors(
        &self,
        errors: &mut ErrorVec,
        fn_ty: TypeId,
        fn_location: Location,
        module_name: &ModuleName,
        arg_pack: TypePackId,
        arg_exprs: &[*mut AstExpr],
        reason: &SubtypingReasoning,
    ) {
        let argument_index = get_argument_index(&reason.sub_path, fn_ty);

        let mut arg_location: Location;
        // If the Nth argument directly corresponds to a term in the AST, use its location.
        if let Some(idx) = argument_index {
            if idx < arg_exprs.len() {
                arg_location = unsafe { (*arg_exprs[idx]).base.location };
            } else if !arg_exprs.is_empty() {
                // Else if any arguments were passed at all, use the location of the last one.
                arg_location = unsafe { (*arg_exprs[arg_exprs.len() - 1]).base.location };
            } else {
                // If no arguments were present, just use the location of the whole function call.
                arg_location = fn_location;
            }
        } else if !arg_exprs.is_empty() {
            arg_location = unsafe { (*arg_exprs[arg_exprs.len() - 1]).base.location };
        } else {
            arg_location = fn_location;
        }

        let prospective_function = unsafe {
            (*self.arena).add_type(FunctionType::function_type_new(
                arg_pack,
                (*self.builtin_types).anyTypePack,
                None,
                false,
            ))
        };

        let failed_sub_pack: Option<TypePackId> = traverse_for_pack(
            prospective_function,
            &reason.super_path,
            unsafe { &*self.builtin_types },
            unsafe { &mut *self.arena },
        );
        let failed_super_pack: Option<TypePackId> = traverse_for_pack(
            fn_ty,
            &reason.sub_path,
            unsafe { &*self.builtin_types },
            unsafe { &mut *self.arena },
        );

        if let Some(fsp) = failed_super_pack {
            if unsafe { !get_type_pack_id::<GenericTypePack>(fsp).is_null() } {
                let given =
                    failed_sub_pack.unwrap_or(unsafe { (*self.builtin_types).emptyTypePack });
                self.maybe_emplace_error_error_vec_location_module_name_subtyping_reasoning_optional_type_pack_id_optional_type_pack_id(
                    errors as *mut ErrorVec,
                    arg_location,
                    module_name,
                    reason as *const SubtypingReasoning,
                    failed_super_pack,
                    Some(given),
                );
                return;
            }
        }

        // If the mismatch is on the argument list itself, then the wrong number of parameters were passed.
        if is_path_on_argument_list(&reason.sub_path) {
            // If insufficiently many parameters are passed, we expect an empty
            // subPath.
            //
            // If too many parameters are passed, we expect a slice subPath which
            // points to the start of the unsatisfied arguments, and a superPath
            // which points at the tail of the parameter list.
            //
            // Sometimes, the superPath includes generic substitutions.  We need to
            // take this into account when computing the expected parameter count.

            if failed_super_pack.is_none() {
                errors.push(TypeError::type_error_location_module_name_type_error_data(
                    fn_location,
                    module_name.clone(),
                    TypeErrorData::InternalError(InternalError::new(
                        "Malformed SubtypingReasoning".to_string(),
                    )),
                ));
                return;
            }

            let required_mapped_args = unsafe {
                (*self.arena).add_type_pack_t(traverse_for_flattened_pack(
                    fn_ty,
                    &reason.sub_path,
                    &*self.builtin_types,
                    &mut *self.arena,
                ))
            };
            let (params_head, _params_tail) = flatten_type_pack_id(required_mapped_args);
            let (arg_head, arg_tail) = flatten_type_pack_id(arg_pack);

            let arg_count = arg_head.len();
            let (_min_params, opt_max_params) =
                get_parameter_extents(TxnLog::empty(), required_mapped_args, false);

            match should_suppress_errors_not_null_normalizer_type_pack_id(self.normalizer, arg_pack)
                .error_suppression_value()
            {
                Value::Suppress => return,
                Value::NormalizationFailed => {
                    errors.push(TypeError::type_error_location_module_name_type_error_data(
                        fn_location,
                        module_name.clone(),
                        TypeErrorData::NormalizationTooComplex(NormalizationTooComplex {
                            _unused: None,
                        }),
                    ));
                    return;
                }
                Value::DoNotSuppress => {}
            }

            // failedSuperPack is guaranteed Some here (checked above).
            match should_suppress_errors_not_null_normalizer_type_pack_id(
                self.normalizer,
                required_mapped_args,
            )
            .error_suppression_value()
            {
                Value::Suppress => return,
                Value::NormalizationFailed => {
                    errors.push(TypeError::type_error_location_module_name_type_error_data(
                        fn_location,
                        module_name.clone(),
                        TypeErrorData::NormalizationTooComplex(NormalizationTooComplex {
                            _unused: None,
                        }),
                    ));
                    return;
                }
                Value::DoNotSuppress => {}
            }

            let is_variadic_flag = match arg_tail {
                Some(t) => is_variadic(t),
                None => false,
            };

            if is_variadic_flag {
                // Not actually a count mismatch!  This can happen if the
                // required parameters are a generic pack that has not been
                // satisfied.
                let given =
                    failed_sub_pack.unwrap_or(unsafe { (*self.builtin_types).emptyTypePack });
                self.maybe_emplace_error_error_vec_location_module_name_subtyping_reasoning_optional_type_pack_id_optional_type_pack_id(
                    errors as *mut ErrorVec,
                    arg_location,
                    module_name,
                    reason as *const SubtypingReasoning,
                    failed_super_pack,
                    Some(given),
                );
            } else {
                errors.push(TypeError::type_error_location_module_name_type_error_data(
                    fn_location,
                    module_name.clone(),
                    TypeErrorData::CountMismatch(CountMismatch {
                        expected: params_head.len(),
                        maximum: opt_max_params,
                        actual: arg_count,
                        context: CountMismatchContext::Arg,
                        is_variadic: is_variadic_flag,
                        function: alloc::string::String::new(),
                    }),
                ));
            }

            return;
        }

        if let Some(idx) = argument_index {
            // If the Nth argument directly corresponds to a term in the AST, use its location.
            if idx < arg_exprs.len() {
                arg_location = unsafe { (*arg_exprs[idx]).base.location };
            } else if !arg_exprs.is_empty() {
                // Else if any arguments were passed at all, use the location of the last one.
                arg_location = unsafe { (*arg_exprs[arg_exprs.len() - 1]).base.location };
            } else {
                // If no arguments were present, just use the location of the whole function call.
                arg_location = fn_location;
            }

            // The first path component should always be PackField::Arguments
            LUAU_ASSERT!(reason.sub_path.components.len() > 1);
            let mut super_path_tail = reason.super_path.clone();
            super_path_tail.components.remove(0);

            let failed_sub = traverse_pack_root(
                arg_pack,
                &super_path_tail,
                unsafe { &*self.builtin_types },
                unsafe { &mut *self.arena },
            );
            let failed_super = traverse_type_root(
                fn_ty,
                &reason.sub_path,
                unsafe { &*self.builtin_types },
                unsafe { &mut *self.arena },
            );

            self.maybe_emplace_error_error_vec_location_module_name_subtyping_reasoning_optional_type_or_pack_optional_type_or_pack(
                errors as *mut ErrorVec,
                arg_location,
                module_name,
                reason as *const SubtypingReasoning,
                failed_super,
                failed_sub,
            );
            return;
        }

        if let Some(fsp) = failed_sub_pack {
            if failed_super_pack.is_none()
                && unsafe { !get_type_pack_id::<GenericTypePack>(fsp).is_null() }
            {
                errors.push(TypeError::type_error_location_module_name_type_error_data(
                    arg_location,
                    module_name.clone(),
                    TypeErrorData::TypePackMismatch(TypePackMismatch {
                        wanted_tp: fsp,
                        given_tp: unsafe { (*self.builtin_types).emptyTypePack },
                        reason: alloc::string::String::new(),
                    }),
                ));
            }
        }

        if let (Some(fsp), Some(fsup)) = (failed_sub_pack, failed_super_pack) {
            // If a bug in type inference occurs, we may have a mismatch in the return packs.
            // This happens when inference incorrectly leaves the result type of a function free.
            // If this happens, we don't want to explode, so we'll use the function's location.
            if arg_exprs.is_empty() {
                arg_location = fn_location;
            } else {
                arg_location = unsafe { (*arg_exprs[arg_exprs.len() - 1]).base.location };
            }

            let error_suppression =
                should_suppress_errors_not_null_normalizer_type_pack_id(self.normalizer, fsp)
                    .or_else(&should_suppress_errors_not_null_normalizer_type_pack_id(
                        self.normalizer,
                        fsup,
                    ));
            if error_suppression.error_suppression_value() == Value::Suppress {
                return;
            }

            match reason.variance {
                SubtypingVariance::Covariant => {
                    errors.push(TypeError::type_error_location_module_name_type_error_data(
                        arg_location,
                        module_name.clone(),
                        TypeErrorData::TypePackMismatch(TypePackMismatch {
                            wanted_tp: fsp,
                            given_tp: fsup,
                            reason: alloc::string::String::new(),
                        }),
                    ));
                }
                SubtypingVariance::Contravariant => {
                    errors.push(TypeError::type_error_location_module_name_type_error_data(
                        arg_location,
                        module_name.clone(),
                        TypeErrorData::TypePackMismatch(TypePackMismatch {
                            wanted_tp: fsup,
                            given_tp: fsp,
                            reason: alloc::string::String::new(),
                        }),
                    ));
                }
                SubtypingVariance::Invariant => {
                    errors.push(TypeError::type_error_location_module_name_type_error_data(
                        arg_location,
                        module_name.clone(),
                        TypeErrorData::TypePackMismatch(TypePackMismatch {
                            wanted_tp: fsp,
                            given_tp: fsup,
                            reason: alloc::string::String::new(),
                        }),
                    ));
                }
                _ => {
                    LUAU_ASSERT!(false);
                }
            }
        }
    }
}
