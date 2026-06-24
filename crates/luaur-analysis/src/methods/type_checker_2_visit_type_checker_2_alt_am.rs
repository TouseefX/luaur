//! Faithful port of `TypeChecker2::visit(AstExprFunction*)` (TypeChecker2.cpp:2026-2148).
use crate::enums::type_context::TypeContext;
use crate::functions::begin_type_pack::begin;
use crate::functions::end_type_pack::end;
use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::to_string_to_string_alt_c::to_string_type_id;
use crate::records::code_too_complex::CodeTooComplex;
use crate::records::extra_information::ExtraInformation;
use crate::records::function_exits_without_returning::FunctionExitsWithoutReturning;
use crate::records::function_type::FunctionType;
use crate::records::generic_error::GenericError;
use crate::records::in_conditional_context::InConditionalContext;
use crate::records::internal_error::InternalError;
use crate::records::never_type::NeverType;
use crate::records::type_checker_2::TypeChecker2;
use crate::type_aliases::error_type::ErrorType;
use alloc::format;
use luaur_ast::records::ast_expr_function::AstExprFunction;

impl TypeChecker2 {
    pub fn visit_ast_expr_function(&mut self, function: *mut AstExprFunction) {
        unsafe {
            let fn_ref = &*function;
            let location = fn_ref.base.base.location;

            // InConditionalContext flipper(&typeContext, TypeContext::Default);
            let _flipper = InConditionalContext::new(&mut self.type_context, TypeContext::Default);

            // auto StackPusher = pushStack(fn);
            let _stack_pusher = self
                .type_checker_2_push_stack(function as *mut luaur_ast::records::ast_node::AstNode);

            self.visit_generics(fn_ref.generics, fn_ref.generic_packs);

            let inferred_fn_ty =
                self.lookup_type(function as *mut luaur_ast::records::ast_expr::AstExpr);
            self.function_decl_stack.push(inferred_fn_ty);

            // std::shared_ptr<const NormalizedType> normalizedFnTy = normalizer.normalize(inferredFnTy);
            let mut normalized_fn_ty = Some(self.normalizer.normalize(inferred_fn_ty));

            if normalized_fn_ty.is_none() {
                self.report_error_type_error_data_location(
                    CodeTooComplex::default().into(),
                    &location,
                );
            } else if !get_type_id::<ErrorType>(normalized_fn_ty.as_ref().unwrap().errors).is_null()
            {
                // If we have an error type, we don't want to do anything else involving the normalized type
                normalized_fn_ty = None;
            } else if !normalized_fn_ty.as_ref().unwrap().has_functions() {
                self.report_error_type_error_data_location(
                    InternalError::new(format!(
                        "Internal error: Lambda has non-function type {}",
                        to_string_type_id(inferred_fn_ty)
                    ))
                    .into(),
                    &location,
                );
                self.function_decl_stack.pop();
                return;
            } else {
                if normalized_fn_ty.as_ref().unwrap().functions.parts.size() != 1 {
                    self.report_error_type_error_data_location(
                        InternalError::new(format!(
                            "Unexpected: Lambda has unexpected type {}",
                            to_string_type_id(inferred_fn_ty)
                        ))
                        .into(),
                        &location,
                    );
                    self.function_decl_stack.pop();
                    return;
                }

                let inferred_ftv_ty = normalized_fn_ty.as_ref().unwrap().functions.parts.front();
                let inferred_ftv = get_type_id::<FunctionType>(inferred_ftv_ty);
                debug_assert!(!inferred_ftv.is_null());
                let inferred_ftv = &*inferred_ftv;

                // There is no way to write an annotation for the self argument, so we
                // cannot do anything to check it.
                let mut arg_it = begin(inferred_ftv.arg_types);
                let arg_end = end(inferred_ftv.arg_types);
                if !fn_ref.self_.is_null() {
                    arg_it.operator_inc();
                }

                let args = fn_ref.args;
                for ai in 0..args.size {
                    if !arg_it.operator_ne(&arg_end) {
                        break;
                    }

                    let arg = *args.data.add(ai);
                    let inferred_arg_ty = *arg_it.operator_deref();

                    if !(*arg).annotation.is_null() {
                        // we need to typecheck any argument annotations themselves.
                        self.visit_ast_type((*arg).annotation);

                        let annotated_arg_ty = self.lookup_annotation((*arg).annotation);

                        self.test_is_subtype_type_id_type_id_location(
                            inferred_arg_ty,
                            annotated_arg_ty,
                            (*arg).location,
                        );
                    }

                    // Some Luau constructs can result in an argument type being
                    // reduced to never by inference. In this case, we want to
                    // report an error at the function, instead of reporting an
                    // error at every callsite.
                    if !get_type_id::<NeverType>(follow_type_id(inferred_arg_ty)).is_null() {
                        // If the annotation simplified to never, we don't want to
                        // even look at contributors.
                        let mut explicitly_never = false;
                        if !(*arg).annotation.is_null() {
                            let annotated_arg_ty = self.lookup_annotation((*arg).annotation);
                            explicitly_never =
                                !get_type_id::<NeverType>(annotated_arg_ty).is_null();
                        }

                        // Not following here is deliberate.
                        let contributors = (*self.module)
                            .upper_bound_contributors
                            .find(&inferred_arg_ty)
                            .cloned();
                        if let Some(contributors) = contributors {
                            if !explicitly_never {
                                let arg_name = core::ffi::CStr::from_ptr((*arg).name.value)
                                    .to_string_lossy()
                                    .into_owned();
                                self.report_error_type_error_data_location(
                                    GenericError::new(format!(
                                        "Parameter '{}' has been reduced to never. This function is not callable with any possible value.",
                                        arg_name
                                    ))
                                    .into(),
                                    &(*arg).location,
                                );
                                for (site, component) in contributors {
                                    self.report_error_type_error_data_location(
                                        ExtraInformation::new(format!(
                                            "Parameter '{}' is required to be a subtype of '{}' here.",
                                            arg_name,
                                            to_string_type_id(component)
                                        ))
                                        .into(),
                                        &site,
                                    );
                                }
                            }
                        }
                    }

                    arg_it.operator_inc();
                }

                // we need to typecheck the vararg annotation, if it exists.
                if fn_ref.vararg && !fn_ref.vararg_annotation.is_null() {
                    self.visit_ast_type_pack(fn_ref.vararg_annotation);
                }

                let reaches_implicit_return = !self
                    .type_checker_2_get_fallthrough(
                        fn_ref.body as *const luaur_ast::records::ast_stat::AstStat,
                    )
                    .is_null();
                if reaches_implicit_return
                    && !self.allows_no_return_values(follow_type_pack_id(inferred_ftv.ret_types))
                {
                    let end_location = self.get_end_location(function);
                    self.report_error_type_error_data_location(
                        FunctionExitsWithoutReturning {
                            expected_return_type: inferred_ftv.ret_types,
                        }
                        .into(),
                        &end_location,
                    );
                }
            }

            self.visit_ast_stat_block(fn_ref.body);

            // we need to typecheck the return annotation itself, if it exists.
            if !fn_ref.return_annotation.is_null() {
                self.visit_ast_type_pack(fn_ref.return_annotation);
            }

            // If the function type has a function annotation, we need to see if we can suggest an annotation
            if let Some(normalized) = normalized_fn_ty.as_ref() {
                let part = normalized.functions.parts.front();
                self.type_checker_2_suggest_annotations(function, part);
            }

            self.function_decl_stack.pop();
        }
    }
}
