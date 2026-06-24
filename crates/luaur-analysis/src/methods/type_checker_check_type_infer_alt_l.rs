//! Faithful port of `ControlFlow TypeChecker::check(const ScopePtr& scope, const AstStatForIn& forin)`
//! (Analysis/src/TypeInfer.cpp:1202-1392).

use crate::enums::control_flow::ControlFlow;
use crate::functions::first::first;
use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_table_type::get_table_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::get_type_pack::get as get_type_pack;
use crate::records::any_type::AnyType;
use crate::records::binding::Binding;
use crate::records::cannot_call_non_function::CannotCallNonFunction;
use crate::records::count_mismatch::CountMismatchContext;
use crate::records::free_type::FreeType;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::function_type::FunctionType;
use crate::records::never_type::NeverType;
use crate::records::symbol::Symbol;
use crate::records::type_checker::TypeChecker;
use crate::records::type_pack::TypePack;
use crate::records::type_pack_var::TypePackVar;
use crate::records::union_type::UnionType;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::error_type_pack::ErrorTypePack;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_for_in::AstStatForIn;
use luaur_ast::records::ast_type_or_pack::AstTypeOrPack;
use luaur_ast::records::location::Location;
use luaur_ast::records::position::Position;
use luaur_ast::rtti::ast_node_as;

impl TypeChecker {
    pub fn check_scope_ptr_ast_stat_for_in(
        &mut self,
        scope: &ScopePtr,
        forin: &AstStatForIn,
    ) -> ControlFlow {
        // ScopePtr loopScope = childScope(scope, forin.location);
        let loop_scope = self.child_scope(scope, &forin.base.base.location);

        // AstLocal** vars = forin.vars.data;
        let vars = forin.vars.data;

        // std::vector<TypeId> varTypes;
        // varTypes.reserve(forin.vars.size);
        let mut var_types: alloc::vec::Vec<TypeId> = alloc::vec::Vec::new();
        var_types.reserve(forin.vars.size);

        for i in 0..forin.vars.size {
            let var = unsafe { *vars.add(i) };

            // AstType* ann = vars[i]->annotation;
            // TypeId ty = ann ? resolveType(scope, *ann) : anyIfNonstrict(freshType(loopScope));
            let ann = unsafe { (*var).annotation };
            let ty = if !ann.is_null() {
                self.resolve_type(scope.clone(), unsafe { &*ann })
            } else {
                let fresh = self.fresh_type_scope_ptr(loop_scope.clone());
                self.any_if_nonstrict(fresh)
            };

            // loopScope->bindings[vars[i]] = {ty, vars[i]->location};
            unsafe {
                let loop_scope_mut =
                    alloc::sync::Arc::as_ptr(&loop_scope) as *mut crate::records::scope::Scope;
                (*loop_scope_mut).bindings.insert(
                    Symbol::from_local(var),
                    Binding {
                        type_id: ty,
                        location: (*var).location,
                        deprecated: false,
                        deprecated_suggestion: alloc::string::String::new(),
                        documentation_symbol: None,
                    },
                );
            }

            // varTypes.push_back(ty);
            var_types.push(ty);
        }

        // AstExpr** values = forin.values.data;
        // AstExpr* firstValue = forin.values.data[0];
        let values = forin.values.data;
        let first_value = unsafe { *values.add(0) };

        // if (!firstValue)
        //     ice("expected at least an iterator function value, but we parsed nothing");
        if first_value.is_null() {
            self.ice_string("expected at least an iterator function value, but we parsed nothing");
        }
        let first_value_ref = unsafe { &*first_value };

        // TypeId iterTy = nullptr;
        // TypePackId callRetPack = nullptr;
        let mut iter_ty: TypeId = core::ptr::null();
        let mut call_ret_pack: TypePackId = core::ptr::null();

        // if (forin.values.size == 1 && firstValue->is<AstExprCall>())
        let first_value_call = unsafe { ast_node_as::<AstExprCall>(first_value as *mut AstNode) };
        if forin.values.size == 1 && !first_value_call.is_null() {
            // AstExprCall* exprCall = firstValue->as<AstExprCall>();
            let expr_call = first_value_call;

            // callRetPack = checkExprPack(scope, *exprCall).type;
            // callRetPack = follow(callRetPack);
            call_ret_pack = self
                .check_expr_pack(scope, unsafe { &*(expr_call as *const AstExpr) })
                .r#type;
            call_ret_pack = unsafe { follow_type_pack_id(call_ret_pack) };

            // if (get<FreeTypePack>(callRetPack))
            if !unsafe { get_type_pack::<FreeTypePack>(call_ret_pack) }.is_null() {
                // iterTy = freshType(scope);
                iter_ty = self.fresh_type_scope_ptr(scope.clone());

                // unify(callRetPack, addTypePack({{iterTy}, freshTypePack(scope)}), scope, forin.location);
                let fresh_tail = self.fresh_type_pack_scope_ptr(scope.clone());
                let expected = self.add_type_pack_vector_type_id_optional_type_pack_id(
                    &alloc::vec![iter_ty],
                    Some(fresh_tail),
                );
                self.unify_type_pack_id_type_pack_id_scope_ptr_location_count_mismatch_context(
                    call_ret_pack,
                    expected,
                    scope,
                    &forin.base.base.location,
                    CountMismatchContext::Arg,
                );
            }
            // else if (get<ErrorTypePack>(callRetPack) || !first(callRetPack))
            else if !unsafe { get_type_pack::<ErrorTypePack>(call_ret_pack) }.is_null()
                || first(call_ret_pack, true).is_none()
            {
                // for (TypeId var : varTypes)
                //     unify(errorRecoveryType(scope), var, scope, forin.location);
                let err_ty = self.error_recovery_type_scope_ptr(scope);
                for &var in &var_types {
                    self.unify_type_id_type_id_scope_ptr_location(
                        err_ty,
                        var,
                        scope,
                        &forin.base.base.location,
                    );
                }

                // return check(loopScope, *forin.body);
                return self.check_scope_ptr_ast_stat_block(&loop_scope, unsafe { &*forin.body });
            }
            // else
            else {
                // iterTy = *first(callRetPack);
                iter_ty = first(call_ret_pack, true).unwrap();
                // iterTy = instantiate(scope, iterTy, exprCall->location);
                iter_ty = self.instantiate(
                    scope,
                    iter_ty,
                    unsafe { (*expr_call).base.base.location },
                    crate::records::txn_log::TxnLog::empty(),
                );
            }
        } else {
            // iterTy = instantiate(scope, checkExpr(scope, *firstValue).type, firstValue->location);
            let checked = self
                .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                    scope,
                    first_value_ref,
                    None,
                    false,
                )
                .r#type;
            iter_ty = self.instantiate(
                scope,
                checked,
                first_value_ref.base.location,
                crate::records::txn_log::TxnLog::empty(),
            );
        }

        // iterTy = stripFromNilAndReport(iterTy, firstValue->location);
        iter_ty = self.strip_from_nil_and_report(iter_ty, &first_value_ref.base.location);

        // if (std::optional<TypeId> iterMM = findMetatableEntry(iterTy, "__iter", firstValue->location, /* addErrors= */ true))
        if self
            .find_metatable_entry(
                iter_ty,
                alloc::string::String::from("__iter"),
                &first_value_ref.base.location,
                true,
            )
            .is_some()
        {
            // if __iter metamethod is present, it will be called and the results are going to be called as if they are functions
            // for (TypeId var : varTypes)
            //     unify(anyType, var, scope, forin.location);
            let any_type = self.any_type;
            for &var in &var_types {
                self.unify_type_id_type_id_scope_ptr_location(
                    any_type,
                    var,
                    scope,
                    &forin.base.base.location,
                );
            }

            // return check(loopScope, *forin.body);
            return self.check_scope_ptr_ast_stat_block(&loop_scope, unsafe { &*forin.body });
        }

        // if (const TableType* iterTable = get<TableType>(iterTy))
        if let Some(iter_table) = get_table_type(iter_ty) {
            // if (iterTable->indexer)
            if let Some(indexer) = iter_table.indexer {
                // if (varTypes.size() > 0)
                //     unify(iterTable->indexer->indexType, varTypes[0], scope, forin.location);
                if var_types.len() > 0 {
                    self.unify_type_id_type_id_scope_ptr_location(
                        indexer.index_type,
                        var_types[0],
                        scope,
                        &forin.base.base.location,
                    );
                }

                // if (varTypes.size() > 1)
                //     unify(iterTable->indexer->indexResultType, varTypes[1], scope, forin.location);
                if var_types.len() > 1 {
                    self.unify_type_id_type_id_scope_ptr_location(
                        indexer.index_result_type,
                        var_types[1],
                        scope,
                        &forin.base.base.location,
                    );
                }

                // for (size_t i = 2; i < varTypes.size(); ++i)
                //     unify(nilType, varTypes[i], scope, forin.location);
                let nil_type = self.nil_type;
                for i in 2..var_types.len() {
                    self.unify_type_id_type_id_scope_ptr_location(
                        nil_type,
                        var_types[i],
                        scope,
                        &forin.base.base.location,
                    );
                }
            } else {
                // for (TypeId var : varTypes)
                //     unify(unknownType, var, scope, forin.location);
                let unknown_type = self.unknown_type;
                for &var in &var_types {
                    self.unify_type_id_type_id_scope_ptr_location(
                        unknown_type,
                        var,
                        scope,
                        &forin.base.base.location,
                    );
                }
            }

            // return check(loopScope, *forin.body);
            return self.check_scope_ptr_ast_stat_block(&loop_scope, unsafe { &*forin.body });
        }

        // const FunctionType* iterFunc = get<FunctionType>(iterTy);
        let iter_func_ptr = unsafe { get_type_id::<FunctionType>(iter_ty) };
        // if (!iterFunc)
        if iter_func_ptr.is_null() {
            // TypeId varTy = get<AnyType>(iterTy) ? anyType : errorRecoveryType(loopScope);
            let var_ty = if !unsafe { get_type_id::<AnyType>(iter_ty) }.is_null() {
                self.any_type
            } else {
                self.error_recovery_type_scope_ptr(&loop_scope)
            };

            // for (TypeId var : varTypes)
            //     unify(varTy, var, scope, forin.location);
            for &var in &var_types {
                self.unify_type_id_type_id_scope_ptr_location(
                    var_ty,
                    var,
                    scope,
                    &forin.base.base.location,
                );
            }

            // if (!get<ErrorType>(iterTy) && !get<AnyType>(iterTy) && !get<FreeType>(iterTy) && !get<NeverType>(iterTy))
            //     reportError(firstValue->location, CannotCallNonFunction{iterTy});
            if unsafe { get_type_id::<ErrorType>(iter_ty) }.is_null()
                && unsafe { get_type_id::<AnyType>(iter_ty) }.is_null()
                && unsafe { get_type_id::<FreeType>(iter_ty) }.is_null()
                && unsafe { get_type_id::<NeverType>(iter_ty) }.is_null()
            {
                self.report_error_location_type_error_data(
                    &first_value_ref.base.location,
                    CannotCallNonFunction { ty: iter_ty }.into(),
                );
            }

            // return check(loopScope, *forin.body);
            return self.check_scope_ptr_ast_stat_block(&loop_scope, unsafe { &*forin.body });
        }
        // We only need the function's argTypes/retTypes; capture them up front so we
        // can keep mutably borrowing `self` for the remaining unifications.
        let iter_func_arg_types = unsafe { (*iter_func_ptr).arg_types };
        let iter_func_ret_types = unsafe { (*iter_func_ptr).ret_types };

        // if (forin.values.size == 1)
        if forin.values.size == 1 {
            // TypePackId argPack = nullptr;
            let arg_pack: TypePackId;

            // if (firstValue->is<AstExprCall>())
            if !first_value_call.is_null() {
                // Extract the remaining return values of the call
                // auto [types, tail] = flatten(callRetPack);
                let (types, tail) = flatten_type_pack_id(call_ret_pack);

                if !types.is_empty() {
                    // std::vector<TypeId> argTypes = std::vector<TypeId>(types.begin() + 1, types.end());
                    // argPack = addTypePack(TypePackVar{TypePack{std::move(argTypes), tail}});
                    let arg_types: alloc::vec::Vec<TypeId> = types[1..].to_vec();
                    arg_pack = self.add_type_pack_type_pack_var(TypePackVar::from(TypePack {
                        head: arg_types,
                        tail,
                    }));
                } else {
                    // argPack = addTypePack(TypePack{});
                    arg_pack = self.add_type_pack_type_pack_var(TypePackVar::from(TypePack {
                        head: alloc::vec::Vec::new(),
                        tail: None,
                    }));
                }
            } else {
                // Check if iterator function accepts 0 arguments
                // argPack = addTypePack(TypePack{});
                arg_pack = self.add_type_pack_type_pack_var(TypePackVar::from(TypePack {
                    head: alloc::vec::Vec::new(),
                    tail: None,
                }));
            }

            // Unifier state = mkUnifier(loopScope, firstValue->location);
            let mut state = self.mk_unifier(&loop_scope, &first_value_ref.base.location);

            // checkArgumentList(loopScope, *firstValue, state, argPack, iterFunc->argTypes, /*argLocations*/ {});
            self.check_argument_list(
                &loop_scope,
                first_value_ref,
                &mut state,
                arg_pack,
                iter_func_arg_types,
                &alloc::vec::Vec::new(),
            );

            // state.log.commit();
            state.log.commit();

            // reportErrors(state.errors);
            let state_errors = state.errors.clone();
            self.report_errors(&state_errors);
        }

        // TypePackId retPack = iterFunc->retTypes;
        let mut ret_pack: TypePackId = iter_func_ret_types;

        // if (forin.values.size >= 2)
        if forin.values.size >= 2 {
            // AstArray<AstExpr*> arguments{forin.values.data + 1, forin.values.size - 1};
            let arguments = AstArray::<*mut AstExpr> {
                data: unsafe { values.add(1) },
                size: forin.values.size - 1,
            };

            // Position start = firstValue->location.begin;
            // Position end = values[forin.values.size - 1]->location.end;
            let start: Position = first_value_ref.base.location.begin;
            let end: Position = unsafe { (**values.add(forin.values.size - 1)).base.location.end };

            // AstExprCall exprCall{Location(start, end), firstValue, arguments, /* self= */ false, AstArray<AstTypeOrPack>{}, Location()};
            let mut expr_call = AstExprCall::new(
                Location::new(start, end),
                first_value,
                arguments,
                false,
                AstArray::<AstTypeOrPack> {
                    data: core::ptr::null_mut(),
                    size: 0,
                },
                Location::default(),
            );

            // retPack = checkExprPack(scope, exprCall).type;
            ret_pack = self
                .check_expr_pack(scope, unsafe {
                    &*(&mut expr_call as *mut AstExprCall as *const AstExpr)
                })
                .r#type;
        }

        // We need to remove 'nil' from the set of options of the first return value
        // if (std::optional<TypeId> fty = first(retPack); fty && !varTypes.empty())
        let fty = first(ret_pack, true);
        if let Some(fty) = fty {
            if !var_types.is_empty() {
                // TypeId keyTy = follow(*fty);
                let mut key_ty = unsafe { follow_type_id(fty) };

                // if (get<UnionType>(keyTy))
                //     if (std::optional<TypeId> ty = tryStripUnionFromNil(keyTy)) keyTy = *ty;
                if !unsafe { get_type_id::<UnionType>(key_ty) }.is_null() {
                    if let Some(stripped) = self.try_strip_union_from_nil(key_ty) {
                        key_ty = stripped;
                    }
                }

                // unify(keyTy, varTypes.front(), scope, forin.location);
                self.unify_type_id_type_id_scope_ptr_location(
                    key_ty,
                    var_types[0],
                    scope,
                    &forin.base.base.location,
                );

                // We have already handled the first variable type, make it match in the pack check
                // varTypes.front() = *fty;
                var_types[0] = fty;
            }
        }

        // TypePackId varPack = addTypePack(TypePackVar{TypePack{std::move(varTypes), freshTypePack(scope)}});
        let fresh_var_tail = self.fresh_type_pack_scope_ptr(scope.clone());
        let var_pack = self
            .add_type_pack_vector_type_id_optional_type_pack_id(&var_types, Some(fresh_var_tail));

        // unify(retPack, varPack, scope, forin.location);
        self.unify_type_pack_id_type_pack_id_scope_ptr_location_count_mismatch_context(
            ret_pack,
            var_pack,
            scope,
            &forin.base.base.location,
            CountMismatchContext::Arg,
        );

        // check(loopScope, *forin.body);
        self.check_scope_ptr_ast_stat_block(&loop_scope, unsafe { &*forin.body });

        // return ControlFlow::None;
        ControlFlow::None
    }
}
