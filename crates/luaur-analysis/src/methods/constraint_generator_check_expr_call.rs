use crate::enums::polarity::Polarity;
use crate::enums::type_context::TypeContext;
use crate::enums::value::Value;
use crate::functions::add_all_as_dependencies::add_all_as_dependencies;
use crate::functions::checkpoint::checkpoint;
use crate::functions::extend_type_pack::extend_type_pack;
use crate::functions::follow_type::follow;
use crate::functions::for_each_constraint::for_each_constraint;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_mutable_type_pack::get_mutable_type_pack_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_table_union::is_table_union;
use crate::functions::match_assert::match_assert;
use crate::functions::match_is_instance_guard::match_is_instance_guard;
use crate::functions::match_set_metatable::match_set_metatable;
use crate::functions::should_suppress_errors_type_utils::should_suppress_errors;
use crate::functions::should_typestate_for_first_argument::should_typestate_for_first_argument;
use crate::records::blocked_type::BlockedType;
use crate::records::blocked_type_pack::BlockedTypePack;
use crate::records::checkpoint::Checkpoint;
use crate::records::constraint::Constraint;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::function_call_constraint::FunctionCallConstraint;
use crate::records::function_check_constraint::FunctionCheckConstraint;
use crate::records::function_type::FunctionType;
use crate::records::in_conditional_context::InConditionalContext;
use crate::records::inference_pack::InferencePack;
use crate::records::metatable_type::MetatableType;
use crate::records::module::Module;
use crate::records::scope::Scope;
use crate::records::symbol::Symbol;
use crate::records::union_builder::UnionBuilder;
use crate::records::union_type::UnionType;
use crate::records::unpack_constraint::UnpackConstraint;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::refinement_id_refinement::RefinementId;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;
use luaur_ast::functions::is_l_value::is_l_value;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_expr_varargs::AstExprVarargs;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;
use luaur_common::FFlag;
use luaur_common::LUAU_ASSERT;

impl ConstraintGenerator {
    pub fn check_expr_call(
        &mut self,
        scope: &ScopePtr,
        call: *mut AstExprCall,
        fn_type: TypeId,
        func_begin: Checkpoint,
        func_end: Checkpoint,
    ) -> InferencePack {
        unsafe {
            let scope_raw: *mut Scope = scope.as_ref() as *const Scope as *mut Scope;

            let mut expr_args: Vec<*mut AstExpr> = Vec::new();

            let mut return_refinements: Vec<RefinementId> = Vec::new();
            let mut discriminant_types: Vec<Option<TypeId>> = Vec::new();

            if (*call).self_ {
                let index_expr = ast_node_as::<AstExprIndexName>((*call).func as *mut AstNode);
                if index_expr.is_null() {
                    (*self.ice).ice_string("method call expression has no 'self'");
                }

                expr_args.push((*index_expr).expr);

                let key = (*self.dfg).get_refinement_key((*index_expr).expr as *const AstExpr);
                if !key.is_null() {
                    let discriminant_ty = (*self.arena).add_type(BlockedType::default());
                    return_refinements.push(
                        self.refinement_arena
                            .implicit_proposition_refinement_key_type_id(key, discriminant_ty),
                    );
                    discriminant_types.push(Some(discriminant_ty));
                } else {
                    discriminant_types.push(None);
                }
            }

            for &arg in (*call).args.iter() {
                expr_args.push(arg);

                let key = (*self.dfg).get_refinement_key(arg as *const AstExpr);
                if !key.is_null() {
                    let discriminant_ty = (*self.arena).add_type(BlockedType::default());
                    return_refinements.push(
                        self.refinement_arena
                            .implicit_proposition_refinement_key_type_id(key, discriminant_ty),
                    );
                    discriminant_types.push(Some(discriminant_ty));
                } else {
                    discriminant_types.push(None);
                }
            }

            let expected_types_for_call: Vec<Option<TypeId>> =
                self.get_expected_call_types_for_function_overloads(fn_type);

            if let Some(module) = &self.module {
                let module_ptr = alloc::sync::Arc::as_ptr(module) as *mut Module;
                *(*module_ptr)
                    .ast_original_call_types
                    .get_or_insert((*call).func as *const AstNode) = fn_type;
            }

            let arg_begin_checkpoint = checkpoint(self as *const ConstraintGenerator);

            let mut args: Vec<TypeId> = Vec::new();
            let mut arg_tail: Option<TypePackId> = None;
            let mut argument_refinements: Vec<RefinementId> = Vec::new();

            for i in 0..expr_args.len() {
                let arg = expr_args[i];

                if i == 0 && (*call).self_ {
                    // The self type has already been computed as a side effect of
                    // computing fnType.  If computing that did not cause us to exceed a
                    // recursion limit, we can fetch it from astTypes rather than
                    // recomputing it.
                    let self_ty: Option<TypeId> = if let Some(module) = &self.module {
                        let module_ptr = alloc::sync::Arc::as_ptr(module) as *mut Module;
                        (*module_ptr)
                            .ast_types
                            .find(&(expr_args[0] as *const AstExpr))
                            .copied()
                    } else {
                        None
                    };
                    if let Some(ty) = self_ty {
                        args.push(ty);
                    } else {
                        args.push(self.fresh_type(scope, Polarity::Negative));
                    }
                } else if i < expr_args.len() - 1
                    || !((*(arg as *mut AstNode)).is::<AstExprCall>()
                        || (*(arg as *mut AstNode)).is::<AstExprVarargs>())
                {
                    let mut expected_type: Option<TypeId> = None;
                    if i < expected_types_for_call.len() {
                        expected_type = expected_types_for_call[i];
                    }
                    if i == 0 && match_assert(&*call) {
                        let _flipper = InConditionalContext::new(
                            &mut self.type_context,
                            TypeContext::Condition,
                        );
                        let inference = self.check_scope_ptr_ast_expr_optional_type_id_bool_bool(
                            scope,
                            arg,
                            expected_type,
                            false,
                            false,
                        );
                        args.push(inference.ty);
                        argument_refinements.push(inference.refinement);
                    } else {
                        let inference = self.check_scope_ptr_ast_expr_optional_type_id_bool_bool(
                            scope,
                            arg,
                            expected_type,
                            false,
                            false,
                        );
                        args.push(inference.ty);
                        argument_refinements.push(inference.refinement);
                    }
                } else {
                    let mut expected_types: Vec<Option<TypeId>> = Vec::new();
                    if i < expected_types_for_call.len() {
                        expected_types.extend_from_slice(&expected_types_for_call[i..]);
                    }
                    let pack = self.check_pack_scope_ptr_ast_expr_vector_optional_type_id_bool(
                        scope,
                        arg,
                        &expected_types,
                        true,
                    );
                    arg_tail = Some(pack.tp);
                    argument_refinements.extend(pack.refinements.iter().copied());
                }
            }

            let arg_end_checkpoint = checkpoint(self as *const ConstraintGenerator);

            if FFlag::DebugLuauUserDefinedClasses.get() {
                let instance_guard = match_is_instance_guard(&*call, &*self.dfg);
                if !instance_guard.is_null() {
                    if args.len() >= 2 {
                        // The class type may not be solved yet (e.g. `A.Point` from a
                        // required module).
                        let objectof_inst = self.create_type_function_instance(
                            &(*self.builtin_types).typeFunctions.objectof_func,
                            alloc::vec![args[1]],
                            Vec::new(),
                            scope,
                            (*call).base.base.location,
                        );
                        return_refinements.push(
                            self.refinement_arena
                                .implicit_proposition_refinement_key_type_id(
                                    instance_guard,
                                    objectof_inst,
                                ),
                        );
                    }
                }
            }

            if match_set_metatable(&*call) {
                let mut arg_tail_pack = crate::records::type_pack::TypePack {
                    head: Vec::new(),
                    tail: None,
                };
                if arg_tail.is_some() && args.len() < 2 {
                    arg_tail_pack = extend_type_pack(
                        &mut *self.arena,
                        self.builtin_types,
                        arg_tail.unwrap(),
                        2 - args.len(),
                        Vec::new(),
                    );
                }

                let mut target: TypeId;
                let mut mt: TypeId;

                if args.len() + arg_tail_pack.head.len() == 2 {
                    target = if args.len() > 0 {
                        args[0]
                    } else {
                        arg_tail_pack.head[0]
                    };
                    mt = if args.len() > 1 {
                        args[1]
                    } else {
                        arg_tail_pack.head[if args.len() == 0 { 1 } else { 0 }]
                    };
                } else {
                    let mut unpacked_types: Vec<TypeId> = Vec::new();
                    if args.len() > 0 {
                        target = follow(args[0]);
                    } else {
                        target = (*self.arena).add_type(BlockedType::default());
                        unpacked_types.push(target);
                    }

                    mt = (*self.arena).add_type(BlockedType::default());
                    unpacked_types.push(mt);

                    let c = self.add_constraint_scope_ptr_location_constraint_v(
                        scope,
                        (*call).base.base.location,
                        ConstraintV::Unpack(UnpackConstraint {
                            result_pack: unpacked_types,
                            source_pack: arg_tail.unwrap(),
                        }),
                    );
                    (*get_mutable_type_id::<BlockedType>(mt)).setOwner(c as *const Constraint);
                    let b = get_mutable_type_id::<BlockedType>(target);
                    if !b.is_null() && (*b).getOwner().is_null() {
                        (*b).setOwner(c as *const Constraint);
                    }
                }

                LUAU_ASSERT!(!target.is_null());
                LUAU_ASSERT!(!mt.is_null());

                target = follow(target);
                if should_suppress_errors(self.normalizer, mt).value == Value::Suppress {
                    mt = (*self.builtin_types).anyType;
                }

                let target_expr = *(*call).args.data.add(0);

                let result_ty: TypeId;

                if is_table_union(target) {
                    let target_union = get_type_id::<UnionType>(target);
                    let mut ub = UnionBuilder::union_builder(self.arena, self.builtin_types);

                    for &ty in &(*target_union).options {
                        ub.add((*self.arena).add_type(MetatableType {
                            table: ty,
                            metatable: mt,
                            syntheticName: None,
                        }));
                    }

                    result_ty = ub.build();
                } else {
                    result_ty = (*self.arena).add_type(MetatableType {
                        table: target,
                        metatable: mt,
                        syntheticName: None,
                    });
                }

                let target_local = ast_node_as::<AstExprLocal>(target_expr as *mut AstNode);
                if !target_local.is_null() {
                    let symbol = Symbol::from_local((*target_local).local);
                    // C++ `scope->bindings[targetLocal->local].typeId = resultTy` — the
                    // operator[] default-constructs a Binding when absent.
                    if let Some(binding) = (*scope_raw).bindings.get_mut(&symbol) {
                        binding.type_id = result_ty;
                    } else {
                        (*scope_raw).bindings.insert(
                            symbol,
                            crate::records::binding::Binding {
                                type_id: result_ty,
                                location: luaur_ast::records::location::Location::default(),
                                deprecated: false,
                                deprecated_suggestion: alloc::string::String::new(),
                                documentation_symbol: None,
                            },
                        );
                    }

                    let def = (*self.dfg).get_def(target_expr as *const AstExpr);
                    *(*scope_raw).lvalue_types.get_or_insert(def) = result_ty; // TODO: typestates: track this as an assignment
                    self.update_r_value_refinements_scope_def_id_type_id(scope_raw, def, result_ty); // TODO: typestates: track this as an assignment

                    // HACK: If we have a targetLocal, it has already been added to the
                    // inferredBindings table.  We want to replace it so that we don't
                    // infer a weird union like tbl | { @metatable something, tbl }
                    let ib_symbol = Symbol::from_local((*target_local).local);
                    if let Some(ib) = self.inferred_bindings.find_mut(&ib_symbol) {
                        ib.types.erase_type_id(target);
                    }

                    self.record_inferred_binding((*target_local).local, result_ty);
                }

                return InferencePack {
                    tp: (*self.arena).add_type_pack_initializer_list_type_id(&[result_ty]),
                    refinements: alloc::vec![self
                        .refinement_arena
                        .variadic_refinement_ids(&return_refinements)],
                };
            }

            if should_typestate_for_first_argument(&*call)
                && (*call).args.size > 0
                && is_l_value(*(*call).args.data.add(0) as *const AstExpr)
            {
                let target_expr = *(*call).args.data.add(0);
                let result_ty = (*self.arena).add_type(BlockedType::default());

                if let Some(def) = (*self.dfg).get_def_optional(target_expr as *const AstExpr) {
                    *(*scope_raw).lvalue_types.get_or_insert(def) = result_ty;
                    self.update_r_value_refinements_scope_def_id_type_id(scope_raw, def, result_ty);
                }
            }

            if match_assert(&*call) && !argument_refinements.is_empty() {
                self.apply_refinements(
                    scope,
                    (**(*call).args.data.add(0)).base.location,
                    argument_refinements[0],
                );
            }

            // TODO: How do expectedTypes play into this?  Do they?
            let rets: TypePackId = (*self.arena).add_type_pack_t(BlockedTypePack {
                index: 0,
                owner: core::ptr::null_mut(),
            });
            let arg_pack: TypePackId = self.add_type_pack(args, arg_tail);
            let ftv = FunctionType::function_type_new(arg_pack, rets, None, (*call).self_);

            let (explicit_type_ids, explicit_type_pack_ids): (Vec<TypeId>, Vec<TypePackId>) =
                if FFlag::LuauExplicitTypeInstantiationSupport.get()
                    && (*call).type_arguments.size != 0
                {
                    self.resolve_type_arguments(scope_raw, (*call).type_arguments)
                } else {
                    (Vec::new(), Vec::new())
                };

            // we don't need ftv after building argPack/rets except to keep the FunctionType
            // shape; the C++ uses `ftv` only via the constraints below which reference
            // fnType/argPack/rets directly.
            let _ = ftv;

            /*
             * To make bidirectional type checking work, we need to solve these constraints in a particular order:
             *
             * 1. Solve the function type
             * 2. Propagate type information from the function type to the argument typeArguments
             * 3. Solve the argument typeArguments
             * 4. Solve the call
             */

            let check_constraint: *mut Constraint = self
                .add_constraint_scope_ptr_location_constraint_v(
                    scope,
                    (*(*call).func).base.location,
                    ConstraintV::FunctionCheck(FunctionCheckConstraint {
                        fn_type,
                        args_pack: arg_pack,
                        call_site: call,
                        ast_types: self
                            .module
                            .as_ref()
                            .map(|m| {
                                let mp = alloc::sync::Arc::as_ptr(m) as *mut Module;
                                &(*mp).ast_types as *const _
                            })
                            .unwrap_or(core::ptr::null()),
                        ast_expected_types: self
                            .module
                            .as_ref()
                            .map(|m| {
                                let mp = alloc::sync::Arc::as_ptr(m) as *mut Module;
                                &(*mp).ast_expected_types as *const _
                            })
                            .unwrap_or(core::ptr::null()),
                    }),
                );

            if FFlag::LuauConstraintGraph.get() {
                add_all_as_dependencies(func_begin, func_end, self, check_constraint);
            } else {
                for_each_constraint(func_begin, func_end, self, |constraint| {
                    (*check_constraint).deprecated_dependencies.push(constraint);
                });
            }

            let call_constraint: *mut Constraint = self
                .add_constraint_scope_ptr_location_constraint_v(
                    scope,
                    (*(*call).func).base.location,
                    ConstraintV::FunctionCall(FunctionCallConstraint {
                        fn_type,
                        args_pack: arg_pack,
                        result: rets,
                        call_site: call,
                        discriminant_types,
                        type_arguments: explicit_type_ids,
                        type_pack_arguments: explicit_type_pack_ids,
                        ast_overload_resolved_types: self
                            .module
                            .as_ref()
                            .map(|m| {
                                let mp = alloc::sync::Arc::as_ptr(m) as *mut Module;
                                &mut (*mp).ast_overload_resolved_types as *mut _
                            })
                            .unwrap_or(core::ptr::null_mut()),
                    }),
                );

            (*get_mutable_type_pack_id::<BlockedTypePack>(rets)).owner = call_constraint;

            if FFlag::LuauConstraintGraph.get() {
                (*self.cgraph).add_dependency_of_constraint_constraint(
                    &mut *check_constraint,
                    &mut *call_constraint,
                );
                for_each_constraint(
                    arg_begin_checkpoint,
                    arg_end_checkpoint,
                    self,
                    |constraint| {
                        (*self.cgraph).add_dependency_of_constraint_constraint(
                            &mut *check_constraint,
                            &mut *constraint,
                        );
                        (*self.cgraph).add_dependency_of_constraint_constraint(
                            &mut *constraint,
                            &mut *call_constraint,
                        );
                    },
                );
            } else {
                (*call_constraint)
                    .deprecated_dependencies
                    .push(check_constraint);
                for_each_constraint(
                    arg_begin_checkpoint,
                    arg_end_checkpoint,
                    self,
                    |constraint| {
                        (*constraint).deprecated_dependencies.push(check_constraint);
                        (*call_constraint).deprecated_dependencies.push(constraint);
                    },
                );
            }

            InferencePack {
                tp: rets,
                refinements: alloc::vec![self
                    .refinement_arena
                    .variadic_refinement_ids(&return_refinements)],
            }
        }
    }
}
