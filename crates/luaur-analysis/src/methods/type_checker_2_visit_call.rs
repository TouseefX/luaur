use alloc::vec::Vec;

use crate::enums::value::Value;
use crate::functions::extend_type_pack::extend_type_pack;
use crate::functions::find_unique_types_ast_utils_alt_d::find_unique_types;
use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_parameter_extents::get_parameter_extents;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_optional::is_optional;
use crate::functions::is_variadic_type_pack::is_variadic;
use crate::functions::report_available_overloads::report_available_overloads;
use crate::functions::should_suppress_errors_type_utils::should_suppress_errors;
use crate::records::ambiguous_function_call::AmbiguousFunctionCall;
use crate::records::any_type::AnyType;
use crate::records::cannot_call_non_function::CannotCallNonFunction;
use crate::records::count_mismatch::{CountMismatch, CountMismatchContext};
use crate::records::function_type::FunctionType;
use crate::records::generic_error::GenericError;
use crate::records::internal_error::InternalError;
use crate::records::intersection_type::IntersectionType;
use crate::records::magic_function_type_check_context::MagicFunctionTypeCheckContext;
use crate::records::multiple_nonviable_overloads::MultipleNonviableOverloads;
use crate::records::never_type::NeverType;
use crate::records::normalization_too_complex::NormalizationTooComplex;
use crate::records::optional_value_access::OptionalValueAccess;
use crate::records::overload_resolver::OverloadResolver;
use crate::records::txn_log::TxnLog;
use crate::records::type_checker_2::TypeChecker2;
use crate::records::type_pack::TypePack;
use crate::records::union_type::UnionType;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use alloc::format;
use alloc::string::String;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_node::AstNode;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::records::variant::Variant2;
use luaur_common::FFlag;

impl TypeChecker2 {
    pub fn visit_call(&mut self, call: *mut AstExprCall) {
        unsafe {
            let module = &mut *self.module;
            let func_node = (*call).func as *const AstNode;
            let Some(original_call_ty) = module.ast_original_call_types.find(&func_node).copied()
            else {
                return;
            };

            let mut fn_ty = follow_type_id(original_call_ty);
            if !get_type_id::<AnyType>(fn_ty).is_null()
                || !get_type_id::<ErrorType>(fn_ty).is_null()
                || !get_type_id::<NeverType>(fn_ty).is_null()
            {
                return;
            }

            if is_optional(fn_ty) {
                match Value::from(should_suppress_errors(&mut self.normalizer, fn_ty)) {
                    Value::Suppress => {}
                    Value::NormalizationFailed => {
                        self.report_error_type_error_data_location(
                            TypeErrorData::NormalizationTooComplex(
                                NormalizationTooComplex::default(),
                            ),
                            &(*(*call).func).base.location,
                        );
                        self.report_error_type_error_data_location(
                            TypeErrorData::OptionalValueAccess(OptionalValueAccess {
                                optional: fn_ty,
                            }),
                            &(*(*call).func).base.location,
                        );
                    }
                    Value::DoNotSuppress => {
                        self.report_error_type_error_data_location(
                            TypeErrorData::OptionalValueAccess(OptionalValueAccess {
                                optional: fn_ty,
                            }),
                            &(*(*call).func).base.location,
                        );
                    }
                }
                return;
            }

            if FFlag::LuauExplicitTypeInstantiationSupport.get() && (*call).type_arguments.size != 0
            {
                self.check_type_instantiation(
                    call as *mut AstExpr,
                    fn_ty,
                    &(*call).base.base.location,
                    (*call).type_arguments,
                );
            }

            let call_node = call as *const AstNode;
            if let Some(selected_overload_ty) =
                module.ast_overload_resolved_types.find(&call_node).copied()
            {
                let scope = self.find_innermost_scope((*call).base.base.location);
                let result = (*self.subtyping).is_subtype_type_id_type_id_not_null_scope(
                    original_call_ty,
                    selected_overload_ty,
                    scope,
                );
                if result.is_subtype {
                    fn_ty = follow_type_id(selected_overload_ty);
                }
                self.report_errors(result.errors);
                if result.normalization_too_complex {
                    return;
                }
            }

            let fty = get_type_id::<FunctionType>(fn_ty);
            if fty.is_null() {
                let mut args = TypePack {
                    head: Vec::<TypeId>::new(),
                    tail: None,
                };
                let mut arg_exprs: Vec<*mut AstExpr> = Vec::new();

                // The `call->self` prelude in C++ `visitCall` runs before the
                // FunctionType/else split (TypeChecker2.cpp:1624-1634), so the
                // method receiver `self` must be prepended onto `args` here too.
                if (*call).self_ {
                    let index_expr = luaur_ast::rtti::ast_node_as::<AstExprIndexName>(
                        (*call).func as *mut AstNode,
                    );
                    if index_expr.is_null() {
                        self.report_error_type_error_data_location(
                            TypeErrorData::InternalError(InternalError {
                                message: "method call expression has no 'self'".into(),
                            }),
                            &(*call).base.base.location,
                        );
                        return;
                    }

                    args.head.push(self.lookup_type((*index_expr).expr));
                    arg_exprs.push((*index_expr).expr);
                }

                for idx in 0..(*call).args.size {
                    let arg_expr = *(*call).args.data.add(idx);
                    arg_exprs.push(arg_expr);

                    if idx == (*call).args.size - 1 {
                        if let Some(last_arg_pack) = module
                            .ast_type_packs
                            .find(&(arg_expr as *const AstExpr))
                            .copied()
                        {
                            let (last_arg_head, last_arg_tail) =
                                flatten_type_pack_id(last_arg_pack);
                            args.head.extend(last_arg_head);
                            args.tail = last_arg_tail;
                            continue;
                        }
                    }

                    if let Some(arg_ty) = module.ast_types.find(&(arg_expr as *const AstExpr)) {
                        args.head.push(*arg_ty);
                    } else if idx == (*call).args.size - 1 {
                        args.tail = Some((*self.builtin_types).anyTypePack);
                    } else {
                        args.head.push((*self.builtin_types).anyType);
                    }
                }

                let args_pack = module.internal_types.add_type_pack_t(args);
                let scope = self.find_innermost_scope((*call).base.base.location);
                let mut resolver = OverloadResolver::new(
                    self.builtin_types,
                    &mut module.internal_types as *mut _,
                    &mut self.normalizer as *mut _,
                    self.type_function_runtime,
                    scope,
                    self.ice,
                    self.limits,
                    (*call).base.base.location,
                );
                let mut unique_types: DenseHashSet<TypeId> =
                    DenseHashSet::new(core::ptr::null_mut());
                find_unique_types(
                    &mut unique_types as *mut DenseHashSet<TypeId>,
                    &arg_exprs,
                    &module.ast_types as *const _,
                );

                let result = resolver.resolve_overload(
                    fn_ty,
                    args_pack,
                    (*(*call).func).base.location,
                    &mut unique_types as *mut DenseHashSet<TypeId>,
                    false,
                );
                if !result.ok.is_empty() {
                    if result.ok.len() > 1 {
                        self.report_error_type_error_data_location(
                            TypeErrorData::AmbiguousFunctionCall(AmbiguousFunctionCall::new(
                                fn_ty, args_pack,
                            )),
                            &(*call).base.base.location,
                        );
                    }
                    return;
                }

                if result.incompatible_overloads.len() == 1 {
                    for (ty, reasons) in result.incompatible_overloads.iter() {
                        match reasons {
                            Variant2::V0(reasonings) => {
                                for reason in reasonings.iter() {
                                    resolver.report_errors(
                                        &mut module.errors,
                                        *ty,
                                        (*(*call).func).base.location,
                                        &module.name,
                                        args_pack,
                                        &arg_exprs,
                                        reason,
                                    );
                                }
                            }
                            Variant2::V1(errors) => {
                                self.report_errors(errors.clone());
                            }
                        }
                    }
                    return;
                }

                if result.incompatible_overloads.len() > 1 {
                    let (arg_head, _) = flatten_type_pack_id(args_pack);
                    let mut overloads_to_report = Vec::new();
                    for (overload_ty, _) in result.incompatible_overloads.iter() {
                        if !self.is_error_suppressing_location_type_id(
                            (*call).base.base.location,
                            *overload_ty,
                        ) {
                            overloads_to_report.push(*overload_ty);
                        }
                    }
                    if !overloads_to_report.is_empty() {
                        self.report_error_type_error_data_location(
                            TypeErrorData::MultipleNonviableOverloads(
                                MultipleNonviableOverloads::new(arg_head.len()),
                            ),
                            &(*call).base.base.location,
                        );
                        report_available_overloads(
                            &mut module.errors,
                            (*call).base.base.location,
                            &module.name,
                            &overloads_to_report,
                        );
                    }
                    return;
                }

                if result.arity_mismatches.len() == 1 {
                    let mismatch_ty = follow_type_id(result.arity_mismatches[0]);
                    let mismatch_fn = get_type_id::<FunctionType>(mismatch_ty);
                    if !mismatch_fn.is_null() {
                        let is_variadic = is_variadic((*mismatch_fn).arg_types);
                        let (min_params, opt_max_params) =
                            get_parameter_extents(TxnLog::empty(), (*mismatch_fn).arg_types, true);
                        let (arg_head, _) = flatten_type_pack_id(args_pack);
                        self.report_error_type_error_data_location(
                            TypeErrorData::CountMismatch(CountMismatch {
                                expected: min_params,
                                maximum: opt_max_params,
                                actual: arg_head.len(),
                                context: CountMismatchContext::Arg,
                                is_variadic,
                                function: String::new(),
                            }),
                            &(*(*call).func).base.location,
                        );
                        return;
                    }
                }

                if !result.arity_mismatches.is_empty() {
                    let (arg_head, _) = flatten_type_pack_id(args_pack);
                    self.report_error_type_error_data_location(
                        TypeErrorData::GenericError(GenericError::new(format!(
                            "No overload for function accepts {} arguments.",
                            arg_head.len()
                        ))),
                        &(*(*call).func).base.location,
                    );
                    report_available_overloads(
                        &mut module.errors,
                        (*(*call).func).base.location,
                        &module.name,
                        &result.arity_mismatches,
                    );
                    return;
                }

                if !result.non_functions.is_empty() {
                    let norm = self.normalizer.try_normalize(fn_ty);
                    let hit_limits = norm.as_ref().is_none_or(|norm| {
                        self.normalizer.is_inhabited_normalized_type(norm.as_ref())
                            == crate::enums::normalization_result::NormalizationResult::HitLimits
                    });
                    if hit_limits {
                        self.report_error_type_error_data_location(
                            TypeErrorData::NormalizationTooComplex(
                                NormalizationTooComplex::default(),
                            ),
                            &(*(*call).func).base.location,
                        );
                    }

                    if norm
                        .as_ref()
                        .is_none_or(|norm| !norm.should_suppress_errors())
                    {
                        self.report_error_type_error_data_location(
                            TypeErrorData::CannotCallNonFunction(CannotCallNonFunction {
                                ty: fn_ty,
                            }),
                            &(*(*call).func).base.location,
                        );
                    }
                } else if get_type_id::<IntersectionType>(fn_ty).is_null()
                    && get_type_id::<UnionType>(fn_ty).is_null()
                {
                    self.report_error_type_error_data_location(
                        TypeErrorData::CannotCallNonFunction(CannotCallNonFunction { ty: fn_ty }),
                        &(*(*call).func).base.location,
                    );
                }
                return;
            }

            let mut arg_exprs: Vec<*mut AstExpr> = Vec::new();
            let self_offset = if (*call).self_ { 1 } else { 0 };
            let use_bidirectional_args = (*fty).generics.is_empty()
                && (*fty).generic_packs.is_empty()
                && (*call).args.size > 0;
            let params_head = if use_bidirectional_args {
                extend_type_pack(
                    &mut module.internal_types,
                    self.builtin_types,
                    (*fty).arg_types,
                    (*call).args.size + self_offset,
                    Vec::new(),
                )
                .head
            } else {
                Vec::new()
            };

            let mut args = TypePack {
                head: Vec::<TypeId>::new(),
                tail: None,
            };

            if (*call).self_ {
                let index_expr =
                    luaur_ast::rtti::ast_node_as::<AstExprIndexName>((*call).func as *mut AstNode);
                if index_expr.is_null() {
                    self.report_error_type_error_data_location(
                        TypeErrorData::InternalError(InternalError {
                            message: "method call expression has no 'self'".into(),
                        }),
                        &(*call).base.base.location,
                    );
                    return;
                }

                args.head.push(self.lookup_type((*index_expr).expr));
                arg_exprs.push((*index_expr).expr);
            }

            for idx in 0..(*call).args.size {
                let arg_expr = *(*call).args.data.add(idx);
                let is_last = idx == (*call).args.size - 1;

                if is_last {
                    if let Some(last_arg_pack) = (*self.module)
                        .ast_type_packs
                        .find(&(arg_expr as *const AstExpr))
                        .copied()
                    {
                        let (last_arg_head, last_arg_tail) = flatten_type_pack_id(last_arg_pack);
                        args.head.extend(last_arg_head);
                        args.tail = last_arg_tail;
                        continue;
                    }
                }

                let arg_expr_type = self.lookup_type(arg_expr);
                arg_exprs.push(arg_expr);
                if use_bidirectional_args
                    && idx + self_offset < params_head.len()
                    && !self.is_error_suppressing_location_type_id(
                        (*arg_expr).base.location,
                        arg_expr_type,
                    )
                {
                    self.test_literal_or_ast_type_is_subtype(
                        arg_expr,
                        params_head[idx + self_offset],
                    );
                    args.head.push(params_head[idx + self_offset]);
                } else {
                    args.head.push(arg_expr_type);
                }
            }

            let args_tp = (*self.module).internal_types.add_type_pack_t(args.clone());
            let original_ftv = get_type_id::<FunctionType>(follow_type_id(original_call_ty));
            if !original_ftv.is_null() {
                if let Some(magic) = (*original_ftv).magic.as_ref() {
                    let scope = self.find_innermost_scope((*call).base.base.location);
                    let used_magic = (magic.type_check)(&MagicFunctionTypeCheckContext {
                        typechecker: core::ptr::NonNull::new_unchecked(self as *mut TypeChecker2),
                        builtin_types: core::ptr::NonNull::new_unchecked(self.builtin_types),
                        call_site: call,
                        arguments: args_tp,
                        check_scope: core::ptr::NonNull::new_unchecked(scope),
                    });

                    if used_magic {
                        return;
                    }
                }
            }

            if args.tail.is_none() {
                let actual = args.head.len();
                let (min_params, opt_max_params) =
                    get_parameter_extents(TxnLog::empty(), (*fty).arg_types, false);

                if actual < min_params
                    || opt_max_params.is_some_and(|max_params| actual > max_params)
                {
                    self.report_error_type_error_data_location(
                        TypeErrorData::CountMismatch(CountMismatch {
                            expected: min_params,
                            maximum: opt_max_params,
                            actual,
                            context: CountMismatchContext::Arg,
                            is_variadic: is_variadic((*fty).arg_types),
                            function: String::new(),
                        }),
                        &(*(*call).func).base.location,
                    );
                    return;
                }
            }

            let args_pack = module.internal_types.add_type_pack_t(args);
            let scope = self.find_innermost_scope((*call).base.base.location);
            let mut resolver = OverloadResolver::new(
                self.builtin_types,
                &mut module.internal_types as *mut _,
                &mut self.normalizer as *mut _,
                self.type_function_runtime,
                scope,
                self.ice,
                self.limits,
                (*call).base.base.location,
            );
            let mut unique_types: DenseHashSet<TypeId> = DenseHashSet::new(core::ptr::null_mut());
            find_unique_types(
                &mut unique_types as *mut DenseHashSet<TypeId>,
                &arg_exprs,
                &module.ast_types as *const _,
            );

            let result = resolver.resolve_overload(
                fn_ty,
                args_pack,
                (*(*call).func).base.location,
                &mut unique_types as *mut DenseHashSet<TypeId>,
                false,
            );

            if !result.potential_overloads.is_empty() {
                self.report_error_type_error_data_location(
                    TypeErrorData::InternalError(InternalError {
                        message:
                            "Internal error: outstanding free or blocked type in function call"
                                .into(),
                    }),
                    &(*call).base.base.location,
                );
            }

            if !result.ok.is_empty() {
                if result.ok.len() > 1 {
                    self.report_error_type_error_data_location(
                        TypeErrorData::AmbiguousFunctionCall(AmbiguousFunctionCall::new(
                            fn_ty, args_pack,
                        )),
                        &(*call).base.base.location,
                    );
                }

                self.lookup_type(call as *mut AstExpr);
                return;
            }

            if result.incompatible_overloads.len() == 1 {
                for (ty, reasons) in result.incompatible_overloads.iter() {
                    match reasons {
                        Variant2::V0(reasonings) => {
                            for reason in reasonings.iter() {
                                resolver.report_errors(
                                    &mut module.errors,
                                    *ty,
                                    (*(*call).func).base.location,
                                    &module.name,
                                    args_pack,
                                    &arg_exprs,
                                    reason,
                                );
                            }
                        }
                        Variant2::V1(errors) => {
                            self.report_errors(errors.clone());
                        }
                    }
                }
                return;
            }

            let (arg_head, _) = flatten_type_pack_id(args_pack);
            if result.incompatible_overloads.len() > 1 {
                let mut overloads_to_report = Vec::new();
                for (overload_ty, _) in result.incompatible_overloads.iter() {
                    if !self.is_error_suppressing_location_type_id(
                        (*call).base.base.location,
                        *overload_ty,
                    ) {
                        overloads_to_report.push(*overload_ty);
                    }
                }

                if !overloads_to_report.is_empty() {
                    self.report_error_type_error_data_location(
                        TypeErrorData::MultipleNonviableOverloads(MultipleNonviableOverloads::new(
                            arg_head.len(),
                        )),
                        &(*call).base.base.location,
                    );
                    report_available_overloads(
                        &mut module.errors,
                        (*call).base.base.location,
                        &module.name,
                        &overloads_to_report,
                    );
                }
                return;
            }

            if result.arity_mismatches.len() == 1 {
                let mismatch_ty = follow_type_id(result.arity_mismatches[0]);
                let mismatch_fn = get_type_id::<FunctionType>(mismatch_ty);
                if !mismatch_fn.is_null() {
                    let is_variadic = is_variadic((*mismatch_fn).arg_types);
                    let (min_params, opt_max_params) =
                        get_parameter_extents(TxnLog::empty(), (*mismatch_fn).arg_types, true);
                    self.report_error_type_error_data_location(
                        TypeErrorData::CountMismatch(CountMismatch {
                            expected: min_params,
                            maximum: opt_max_params,
                            actual: arg_head.len(),
                            context: CountMismatchContext::Arg,
                            is_variadic,
                            function: String::new(),
                        }),
                        &(*(*call).func).base.location,
                    );
                    return;
                }
            }

            if !result.arity_mismatches.is_empty() {
                self.report_error_type_error_data_location(
                    TypeErrorData::GenericError(GenericError::new(format!(
                        "No overload for function accepts {} arguments.",
                        arg_head.len()
                    ))),
                    &(*(*call).func).base.location,
                );
                report_available_overloads(
                    &mut module.errors,
                    (*(*call).func).base.location,
                    &module.name,
                    &result.arity_mismatches,
                );
                return;
            }

            if !result.non_functions.is_empty() {
                let norm = self.normalizer.try_normalize(fn_ty);
                let hit_limits = norm.as_ref().is_none_or(|norm| {
                    self.normalizer.is_inhabited_normalized_type(norm.as_ref())
                        == crate::enums::normalization_result::NormalizationResult::HitLimits
                });
                if hit_limits {
                    self.report_error_type_error_data_location(
                        TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
                        &(*(*call).func).base.location,
                    );
                }

                if norm
                    .as_ref()
                    .is_none_or(|norm| !norm.should_suppress_errors())
                {
                    self.report_error_type_error_data_location(
                        TypeErrorData::CannotCallNonFunction(CannotCallNonFunction { ty: fn_ty }),
                        &(*(*call).func).base.location,
                    );
                }
            }
        }
    }
}
