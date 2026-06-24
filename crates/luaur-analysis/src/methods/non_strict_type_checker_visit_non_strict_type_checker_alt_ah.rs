use crate::enums::value_context::ValueContext;
use crate::functions::begin_type_pack::begin_type_pack_id;
use crate::functions::end_type_pack::end_type_pack_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_or_pack::follow_type_or_pack;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::follow_type_utils::follow_optional_ty;
use crate::functions::get_function_name_as_string::get_function_name_as_string;
use crate::functions::is_optional::is_optional;
use crate::records::any_type::AnyType;
use crate::records::any_type::AnyType as AnyTypeRecord;
use crate::records::ast_expr::AstExpr;
use crate::records::checked_function_call_error::CheckedFunctionCallError;
use crate::records::checked_function_incorrect_args::CheckedFunctionIncorrectArgs;
use crate::records::function_type::FunctionType;
use crate::records::negation_type::NegationType;
use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use crate::records::normalization_too_complex::NormalizationTooComplex;
use crate::records::normalized_type::NormalizedType;
use crate::records::scope::Scope;
use crate::records::type_pack::TypePack;
use crate::records::type_pack_iterator::TypePackIterator;
use crate::records::type_pack_iterator::TypePackIterator as TypePackIteratorAlias;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::def_id_def::DefId;
use crate::type_aliases::def_id_def::DefId as DefIdAlias;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_id::TypeId as TypeIdAlias;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_id::TypePackId as TypePackIdAlias;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;

use luaur_common::FFlag;

impl NonStrictTypeChecker {
    pub fn visit_ast_expr_call(&mut self, call: *mut AstExprCall) -> NonStrictContext {
        unsafe {
            // visit(call->func, ValueContext::RValue);
            let func_ptr = (*call).func;
            self.visit_ast_expr_value_context(func_ptr, ValueContext::RValue);

            // for (auto arg : call->args) visit(arg, ValueContext::RValue);
            for i in 0..(*call).args.size {
                let arg = *(*call).args.data.add(i);
                self.visit_ast_expr_value_context(arg, ValueContext::RValue);
            }

            let fresh = NonStrictContext::non_strict_context();
            // C++ `TypeId* originalCallTy = module->astOriginalCallTypes.find(call->func);`
            // (keyed by `const AstNode*`); `if (!originalCallTy) return fresh;`
            let call_func_node = (*call).func as *const luaur_ast::records::ast_node::AstNode;
            let original_call_ty =
                match (*self.module).ast_original_call_types.find(&call_func_node) {
                    Some(v) => v,
                    None => return fresh,
                };

            let fn_ty = *original_call_ty;
            // if (auto fn = get<FunctionType>(follow(fnTy)); fn && fn->isCheckedFunction)
            let followed_fn_ty = follow_type_id(fn_ty);
            let fn_ptr =
                crate::functions::get_type_alt_j::get_type_id::<FunctionType>(followed_fn_ty);
            if fn_ptr.is_null() {
                return fresh;
            }
            if !(*fn_ptr).is_checked_function {
                return fresh;
            }

            // Build argument list
            let mut arguments: alloc::vec::Vec<*mut AstExpr> = alloc::vec::Vec::with_capacity(
                (*call).args.size + if (*call).self_ { 1 } else { 0 },
            );

            if (*call).self_ {
                // C++ `if (auto indexExpr = call->func->as<AstExprIndexName>())`
                let index_expr = luaur_ast::rtti::ast_node_as::<AstExprIndexName>(
                    (*call).func as *mut luaur_ast::records::ast_node::AstNode,
                );
                if !index_expr.is_null() {
                    arguments.push((*index_expr).expr);
                } else {
                    (*self.ice).ice_string("method call expression has no 'self'");
                }
            }

            if (*call).args.size > 0 {
                arguments.extend_from_slice(core::slice::from_raw_parts(
                    (*call).args.data,
                    (*call).args.size,
                ));
            }

            // Collect expected arg types
            let mut arg_types: alloc::vec::Vec<TypeIdAlias> = alloc::vec::Vec::new();
            arg_types.reserve(arguments.len());

            let mut curr: TypePackIteratorAlias =
                crate::functions::begin_type_pack::begin_type_pack_id((*fn_ptr).arg_types);
            let fin: TypePackIteratorAlias =
                crate::functions::end_type_pack::end_type_pack_id((*fn_ptr).arg_types);
            while curr.operator_ne(&fin) {
                let ty: TypeIdAlias = *curr.operator_deref();
                arg_types.push(ty);
                curr.operator_inc();
            }

            if let Some(arg_tail) = curr.tail() {
                let followed = follow_type_pack_id(arg_tail);
                let vtp_ptr =
                    crate::functions::get_type_pack::get_type_pack_id::<VariadicTypePack>(followed);
                if !vtp_ptr.is_null() {
                    while arg_types.len() < arguments.len() {
                        arg_types.push((*vtp_ptr).ty);
                    }
                }
            }

            let function_name = get_function_name_as_string(&*(*call).func)
                .unwrap_or_else(|| alloc::string::String::new());

            if arguments.len() > arg_types.len() {
                self.report_error(
                    TypeErrorData::CheckedFunctionIncorrectArgs(CheckedFunctionIncorrectArgs::new(
                        function_name,
                        arg_types.len(),
                        arguments.len(),
                    )),
                    &(*call).base.base.location,
                );
                return fresh;
            }

            let mut fresh_ctx = NonStrictContext::non_strict_context();
            for i in 0..arguments.len() {
                let arg = arguments[i];
                let expected_arg_type = arg_types[i];
                // C++ `std::shared_ptr<const NormalizedType> norm = normalizer.normalize(...)`.
                // The landed `normalize` returns an always-present `Arc<NormalizedType>`
                // (never null), so the C++ `if (!norm) reportError(NormalizationTooComplex)`
                // branch is unreachable here.
                let norm = self.normalizer.normalize(expected_arg_type);

                let run_time_error_ty: TypeIdAlias;
                let any_ptr =
                    crate::functions::get_type_alt_j::get_type_id::<AnyTypeRecord>(norm.tops);
                if !any_ptr.is_null() {
                    run_time_error_ty = (*self.builtin_types).neverType;
                } else {
                    run_time_error_ty = self.get_or_create_negation(expected_arg_type);
                }

                let def: DefIdAlias = (*self.dfg).get_def(arg);
                fresh_ctx.add_context(&def, run_time_error_ty);
            }

            let scope = self.find_innermost_scope((*call).base.base.location);
            for i in 0..arguments.len() {
                let arg = arguments[i];
                if let Some(run_time_failure_type) =
                    self.will_run_time_error(arg, &fresh_ctx, scope)
                {
                    self.report_error(
                        TypeErrorData::CheckedFunctionCallError(CheckedFunctionCallError::new(
                            arg_types[i],
                            run_time_failure_type,
                            function_name.clone(),
                            i,
                        )),
                        &(*arg).base.location,
                    );
                }
            }

            if arguments.len() < arg_types.len() {
                let mut remaining_args_optional = true;
                for i in arguments.len()..arg_types.len() {
                    remaining_args_optional = remaining_args_optional && is_optional(arg_types[i]);
                }

                if !remaining_args_optional {
                    self.report_error(
                        TypeErrorData::CheckedFunctionIncorrectArgs(
                            CheckedFunctionIncorrectArgs::new(
                                function_name,
                                arg_types.len(),
                                arguments.len(),
                            ),
                        ),
                        &(*call).base.base.location,
                    );
                    return fresh_ctx;
                }
            }

            return fresh_ctx;
        }
    }
}
