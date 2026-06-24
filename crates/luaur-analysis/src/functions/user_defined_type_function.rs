//! Node: `cxx:Function:Luau.Analysis:Analysis/src/UserDefinedTypeFunction.cpp:219:userDefinedTypeFunction`
//! Source: `Analysis/src/UserDefinedTypeFunction.cpp:219-494`
//!
//! Faithful port of `TypeFunctionReductionResult<TypeId> userDefinedTypeFunction(...)`.
//! Evaluates a user-defined ("user") type function by running its compiled body
//! on a sandboxed Luau VM thread: it checks for blocking (pending) types,
//! registers the visible environment, serializes the type arguments into Lua
//! type userdata, calls the function under `lua_pcall`, then deserializes the
//! returned type userdata back into a `TypeId`.
//!
//! Signature is aligned to the canonical `ReducerFunction` fn-pointer shape
//! (by-value vecs + `*mut TypeFunctionContext`) so it can be wired into
//! `BuiltinTypeFunctions::user_func`.
use crate::enums::reduction::Reduction;
use crate::functions::check_result_for_error::check_result_for_error;
use crate::functions::check_result_for_error_deprecated::check_result_for_error_deprecated;
use crate::functions::deserialize_type_function_runtime_builder::deserialize_type_function_type_id_type_function_runtime_builder_state;
use crate::functions::follow_type::follow;
use crate::functions::get_mutable_type::getMutable;
use crate::functions::get_type_user_data::get_type_user_data;
use crate::functions::is_pending::is_pending;
use crate::functions::is_type_user_data::is_type_user_data;
use crate::functions::reset_type_function_state::reset_type_function_state;
use crate::functions::serialize_type_function_runtime_builder::serialize_type_id_type_function_runtime_builder_state;
use crate::functions::to_string_type_function_error::to_string;
use crate::records::extern_type::ExternType;
use crate::records::find_user_type_function_blockers::FindUserTypeFunctionBlockers;
use crate::records::freeze_type_function_types::FreezeTypeFunctionTypes;
use crate::records::generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait};
use crate::records::luau_temp_thread_popper::LuauTempThreadPopper;
use crate::records::scoped_assign::ScopedAssign;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_reduction_result::TypeFunctionReductionResult;
use crate::records::type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState;
use crate::records::type_function_type::TypeFunctionType;
use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::boxed::Box;
use alloc::ffi::CString;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::ffi::c_void;
use core::ptr::NonNull;
use luaur_common::records::dense_hash_set::DenseHashSet;

// `FindUserTypeFunctionBlockers` overrides the bare type/type-pack `visit`s of
// its `TypeOnceVisitor` (`GenericTypeVisitor`) base. To make the base's
// `traverse(...)` dispatch through those overrides, the visitor must implement
// `GenericTypeVisitorTrait`. The overrides already exist as inherent methods
// (see `methods/find_user_type_function_blockers_visit_user_defined_type_function*`);
// this trait impl just forwards to them.
impl GenericTypeVisitorTrait for FindUserTypeFunctionBlockers {
    type Seen = DenseHashSet<*mut c_void>;

    fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
        &mut self.base.base
    }

    fn visit_type_id(&mut self, ty: TypeId) -> bool {
        FindUserTypeFunctionBlockers::visit_type_id(self, ty)
    }

    fn visit_type_pack_id(&mut self, tp: TypePackId) -> bool {
        FindUserTypeFunctionBlockers::visit_type_pack_id(self, tp)
    }

    fn visit_type_id_extern_type(&mut self, ty: TypeId, etv: &ExternType) -> bool {
        FindUserTypeFunctionBlockers::visit_type_id_extern_type(self, ty, etv)
    }
}

// Interrupt handler for type functions: respects type checking limits and LSP
// cancellation requests. C++ throws `TimeLimitError`/`UserCancelError`; the Rust
// port surfaces those via `panic!` (the same mechanism the ported throw-methods
// use), unwinding out of the VM call.
unsafe extern "C-unwind" fn user_defined_type_function_interrupt(
    l: *mut luaur_vm::records::lua_state::lua_State,
    _gc: core::ffi::c_int,
) {
    let main = luaur_vm::functions::lua_mainthread::lua_mainthread(l);
    let data = luaur_vm::functions::lua_getthreaddata::lua_getthreaddata(main);
    let ctx = data as *const crate::records::type_function_runtime::TypeFunctionRuntime;

    if let Some(finish_time) = (*ctx).limits.finishTime {
        if luaur_common::functions::get_clock::get_clock() > finish_time {
            panic!(
                "{}",
                crate::records::time_limit_error::TimeLimitError::time_limit_error_time_limit_error(
                    &(*ctx).ice.module_name
                )
            );
        }
    }

    if let Some(token) = &(*ctx).limits.cancellationToken {
        if token.requested() {
            panic!(
                "{}",
                crate::records::user_cancel_error::UserCancelError::new(
                    (*ctx).ice.module_name.clone()
                )
            );
        }
    }
}

#[allow(non_snake_case)]
pub fn user_defined_type_function(
    instance: TypeId,
    type_params: Vec<TypeId>,
    _pack_params: Vec<TypePackId>,
    ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
    unsafe {
        let ctx_ptr = ctx;

        // auto typeFunction = getMutable<TypeFunctionInstanceType>(instance);
        // LUAU_ASSERT(typeFunction);
        let type_function = getMutable::<TypeFunctionInstanceType>(instance);
        luaur_common::macros::luau_assert::LUAU_ASSERT!(!type_function.is_null());

        // if (typeFunction->userFuncData.owner.expired())
        if (*type_function).user_func_data.owner.upgrade().is_none() {
            (*ctx_ptr)
                .ice
                .as_ref()
                .ice_string("user-defined type function module has expired");
            return TypeFunctionReductionResult {
                result: None,
                reduction_status: Reduction::Erroneous,
                blocked_types: Vec::new(),
                blocked_packs: Vec::new(),
                error: None,
                messages: Vec::new(),
            };
        }

        // if (!typeFunction->userFuncName || !typeFunction->userFuncData.definition)
        if (*type_function).user_func_name.is_none()
            || (*type_function).user_func_data.definition.is_null()
        {
            (*ctx_ptr).ice.as_ref().ice_string(
                "all user-defined type functions must have an associated function definition",
            );
            return TypeFunctionReductionResult {
                result: None,
                reduction_status: Reduction::Erroneous,
                blocked_types: Vec::new(),
                blocked_packs: Vec::new(),
                error: None,
                messages: Vec::new(),
            };
        }

        let type_function_runtime = (*ctx_ptr).type_function_runtime.as_ptr();

        // If type functions cannot be evaluated because of errors in the code, we do not generate any additional ones
        // if (!ctx->typeFunctionRuntime->allowEvaluation || typeFunction->userFuncData.definition->hasErrors)
        if !(*type_function_runtime).allow_evaluation
            || (*(*type_function).user_func_data.definition).has_errors
        {
            return TypeFunctionReductionResult {
                result: Some(error_type(ctx_ptr)),
                reduction_status: Reduction::MaybeOk,
                blocked_types: Vec::new(),
                blocked_packs: Vec::new(),
                error: None,
                messages: Vec::new(),
            };
        }

        // FindUserTypeFunctionBlockers check{ctx};
        let mut check = FindUserTypeFunctionBlockers::find_user_type_function_blockers(
            NonNull::new_unchecked(ctx_ptr),
        );

        // for (auto typeParam : typeParams) check.traverse(follow(typeParam));
        for &type_param in type_params.iter() {
            check.traverse_type_id(follow(type_param));
        }

        // Check that our environment doesn't depend on any type aliases that are blocked
        // for (auto& [name, definition] : typeFunction->userFuncData.environmentAlias)
        //     if (definition.first->typeParams.empty() && definition.first->typePackParams.empty())
        //         check.traverse(follow(definition.first->type));
        {
            let alias_entries: Vec<*mut crate::records::type_fun::TypeFun> = (*type_function)
                .user_func_data
                .environment_alias
                .iter()
                .map(|(_name, def)| def.0)
                .collect();
            for tf in alias_entries {
                if (*tf).type_params().is_empty() && (*tf).type_pack_params().is_empty() {
                    check.traverse_type_id(follow((*tf).r#type()));
                }
            }
        }

        // if (!check.blockingTypes.empty())
        //     return {std::nullopt, Reduction::MaybeOk, check.blockingTypes, {}};
        if !check.blocking_types.is_empty() {
            return TypeFunctionReductionResult {
                result: None,
                reduction_status: Reduction::MaybeOk,
                blocked_types: check.blocking_types.clone(),
                blocked_packs: Vec::new(),
                error: None,
                messages: Vec::new(),
            };
        }

        // Ensure that whole type function environment is registered
        // for (auto& [name, definition] : typeFunction->userFuncData.environmentFunction)
        {
            let func_entries: Vec<(
                *mut luaur_ast::records::ast_stat_type_function::AstStatTypeFunction,
                usize,
            )> = (*type_function)
                .user_func_data
                .environment_function
                .iter()
                .map(|(_name, def)| (def.0, def.1))
                .collect();

            for (def_ptr, _depth) in func_entries {
                // Cannot evaluate if a potential dependency couldn't be parsed
                // if (definition.first->hasErrors)
                if (*def_ptr).has_errors {
                    return TypeFunctionReductionResult {
                        result: Some(error_type(ctx_ptr)),
                        reduction_status: Reduction::MaybeOk,
                        blocked_types: Vec::new(),
                        blocked_packs: Vec::new(),
                        error: None,
                        messages: Vec::new(),
                    };
                }

                // bool registrationFailed = ... registerFunction(definition.first).has_value()
                let registration_failed =
                    if luaur_common::FFlag::LuauTypeFunctionStructuredErrors.get() {
                        (*type_function_runtime)
                            .register_function(def_ptr)
                            .is_some()
                    } else {
                        (*type_function_runtime)
                            .register_function_deprecated(def_ptr)
                            .is_some()
                    };
                if registration_failed {
                    // Failure to register at this point means that original definition had to error out and should not
                    // have been present in the environment
                    (*ctx_ptr)
                        .ice
                        .as_ref()
                        .ice_string("user-defined type function reference cannot be registered");
                    return TypeFunctionReductionResult {
                        result: None,
                        reduction_status: Reduction::Erroneous,
                        blocked_types: Vec::new(),
                        blocked_packs: Vec::new(),
                        error: None,
                        messages: Vec::new(),
                    };
                }
            }
        }

        // AstName name = typeFunction->userFuncData.definition->name;
        let name = (*(*type_function).user_func_data.definition).name;
        let name_str = ast_name_to_string(name.value);

        // lua_State* global = ctx->typeFunctionRuntime->state.get();
        let global = (*type_function_runtime).state.0;

        // if (global == nullptr)
        //     return {..., format("'%s' type function: cannot be evaluated in this context", name.value)};
        if global.is_null() {
            return TypeFunctionReductionResult {
                result: None,
                reduction_status: Reduction::Erroneous,
                blocked_types: Vec::new(),
                blocked_packs: Vec::new(),
                error: Some(luaur_common::functions::format::format(format_args!(
                    "'{}' type function: cannot be evaluated in this context",
                    name_str
                ))),
                messages: Vec::new(),
            };
        }

        // Separate sandboxed thread for individual execution and private globals
        // lua_State* L = lua_newthread(global);
        // LuauTempThreadPopper popper(global);
        let l_vm = luaur_vm::functions::lua_newthread::lua_newthread(
            global as *mut luaur_vm::type_aliases::lua_state::lua_State,
        );
        let l = l_vm as *mut lua_State;
        let mut popper = LuauTempThreadPopper::new(global);

        // std::unique_ptr<TypeFunctionRuntimeBuilderState> runtimeBuilder = std::make_unique<...>(ctx);
        let mut runtime_builder: Box<TypeFunctionRuntimeBuilderState> =
            Box::new(TypeFunctionRuntimeBuilderState::new(ctx_ptr));
        let runtime_builder_ptr: *mut TypeFunctionRuntimeBuilderState = runtime_builder.as_mut();

        // ScopedAssign setRuntimeBuilder(ctx->typeFunctionRuntime->runtimeBuilder, runtimeBuilder.get());
        let _set_runtime_builder = ScopedAssign::new(
            &mut (*type_function_runtime).runtime_builder,
            runtime_builder_ptr,
        );
        // ScopedAssign enableReduction(ctx->normalizer->sharedState->reentrantTypeReduction, false);
        let shared_state = (*(*ctx_ptr).normalizer.as_ptr()).shared_state;
        let _enable_reduction =
            ScopedAssign::new(&mut (*shared_state).reentrant_type_reduction, false);

        // Build up the environment table of each function we have visible
        // for (auto& [_, curr] : typeFunction->userFuncData.environmentFunction)
        let curr_entries: Vec<(
            *mut luaur_ast::records::ast_stat_type_function::AstStatTypeFunction,
            usize,
        )> = (*type_function)
            .user_func_data
            .environment_function
            .iter()
            .map(|(_name, def)| (def.0, def.1))
            .collect();

        for (curr_ptr, curr_depth) in curr_entries {
            // Environment table has to be filled only once in the current execution context
            // if (ctx->typeFunctionRuntime->initialized.find(curr.first)) continue;
            if (*type_function_runtime)
                .initialized
                .find(&curr_ptr)
                .is_some()
            {
                continue;
            }
            // ctx->typeFunctionRuntime->initialized.insert(curr.first);
            (*type_function_runtime).initialized.insert(curr_ptr);

            // lua_pushlightuserdata(L, curr.first);
            // lua_gettable(L, LUA_REGISTRYINDEX);
            luaur_vm::macros::lua_pushlightuserdata::lua_pushlightuserdata(
                l as *mut c_void,
                curr_ptr as *mut c_void,
            );
            luaur_vm::functions::lua_gettable::lua_gettable(
                l_vm,
                luaur_vm::macros::lua_registryindex::LUA_REGISTRYINDEX,
            );

            // if (!lua_isfunction(L, -1))
            if !luaur_vm::lua_isfunction!(l_vm, -1) {
                (*ctx_ptr).ice.as_ref().ice_string(
                    "user-defined type function reference cannot be found in the registry",
                );
                return TypeFunctionReductionResult {
                    result: None,
                    reduction_status: Reduction::Erroneous,
                    blocked_types: Vec::new(),
                    blocked_packs: Vec::new(),
                    error: None,
                    messages: Vec::new(),
                };
            }

            // Build up the environment of the current function, where some might not be visible
            // lua_getfenv(L, -1);
            // lua_setreadonly(L, -1, false);
            luaur_vm::functions::lua_getfenv::lua_getfenv(l_vm, -1);
            luaur_vm::functions::lua_setreadonly::lua_setreadonly(l_vm, -1, 0);

            // for (auto& [name, definition] : typeFunction->userFuncData.environmentFunction)
            let func_env: Vec<(
                CString,
                *mut luaur_ast::records::ast_stat_type_function::AstStatTypeFunction,
                usize,
            )> = (*type_function)
                .user_func_data
                .environment_function
                .iter()
                .map(|(name, def)| (CString::new(name.as_bytes()).unwrap(), def.0, def.1))
                .collect();

            for (name_c, def_ptr, def_depth) in func_env.iter() {
                // Filter visibility based on original scope depth
                // if (definition.second >= curr.second)
                if *def_depth >= curr_depth {
                    // lua_pushlightuserdata(L, definition.first);
                    // lua_gettable(L, LUA_REGISTRYINDEX);
                    luaur_vm::macros::lua_pushlightuserdata::lua_pushlightuserdata(
                        l as *mut c_void,
                        *def_ptr as *mut c_void,
                    );
                    luaur_vm::functions::lua_gettable::lua_gettable(
                        l_vm,
                        luaur_vm::macros::lua_registryindex::LUA_REGISTRYINDEX,
                    );

                    // if (!lua_isfunction(L, -1)) break;
                    if !luaur_vm::lua_isfunction!(l_vm, -1) {
                        break; // Don't have to report an error here, we will visit each function in outer loop
                    }

                    // lua_setfield(L, -2, name.c_str());
                    luaur_vm::functions::lua_setfield::lua_setfield(l_vm, -2, name_c.as_ptr());
                }
            }

            // for (auto& [name, definition] : typeFunction->userFuncData.environmentAlias)
            let alias_env: Vec<(CString, *mut crate::records::type_fun::TypeFun, usize)> =
                (*type_function)
                    .user_func_data
                    .environment_alias
                    .iter()
                    .map(|(name, def)| (CString::new(name.as_bytes()).unwrap(), def.0, def.1))
                    .collect();

            for (name_c, def_ptr, def_depth) in alias_env.iter() {
                // Filter visibility based on original scope depth
                // if (definition.second >= curr.second)
                if *def_depth >= curr_depth {
                    // if (definition.first->typeParams.empty() && definition.first->typePackParams.empty())
                    if (**def_ptr).type_params().is_empty()
                        && (**def_ptr).type_pack_params().is_empty()
                    {
                        // TypeId ty = follow(definition.first->type);
                        let ty = follow((**def_ptr).r#type());

                        // This is checked at the top of the function, and should still be true.
                        // LUAU_ASSERT(!isPending(ty, ctx->solver));
                        luaur_common::macros::luau_assert::LUAU_ASSERT!(!is_pending(
                            ty,
                            (*ctx_ptr).solver
                        ));

                        // TypeFunctionTypeId serializedTy = serialize(ty, runtimeBuilder.get());
                        let serialized_ty: TypeFunctionTypeId =
                            serialize_type_id_type_function_runtime_builder_state(
                                ty,
                                runtime_builder_ptr,
                            );

                        if luaur_common::FFlag::LuauTypeFunctionRobustness.get() {
                            // Only register aliases that are representable in type environment
                            // if (serializedTy && (... ? errors.empty() : errors_DEPRECATED.empty()))
                            let errors_empty =
                                if luaur_common::FFlag::LuauTypeFunctionStructuredErrors.get() {
                                    runtime_builder.errors.is_empty()
                                } else {
                                    runtime_builder.errors_deprecated.is_empty()
                                };
                            if !serialized_ty.is_null() && errors_empty {
                                if luaur_common::FFlag::LuauTypeFunctionSupportsFrozen.get() {
                                    let mut freezer = FreezeTypeFunctionTypes::new();
                                    freezer.base.run_type_function_type_id(serialized_ty);
                                }

                                // allocTypeUserData(L, serializedTy->type, /* frozen */ true);
                                let variant = (*(serialized_ty as *mut TypeFunctionType))
                                    .type_variant
                                    .clone();
                                crate::functions::alloc_type_user_data::alloc_type_user_data(
                                    l, variant, true,
                                );
                                // lua_setfield(L, -2, name.c_str());
                                luaur_vm::functions::lua_setfield::lua_setfield(
                                    l_vm,
                                    -2,
                                    name_c.as_ptr(),
                                );
                            }
                        } else {
                            if luaur_common::FFlag::LuauTypeFunctionSupportsFrozen.get() {
                                let mut freezer = FreezeTypeFunctionTypes::new();
                                freezer.base.run_type_function_type_id(serialized_ty);
                            }

                            // Only register aliases that are representable in type environment
                            let errors_empty =
                                if luaur_common::FFlag::LuauTypeFunctionStructuredErrors.get() {
                                    runtime_builder.errors.is_empty()
                                } else {
                                    runtime_builder.errors_deprecated.is_empty()
                                };
                            if errors_empty {
                                let variant = (*(serialized_ty as *mut TypeFunctionType))
                                    .type_variant
                                    .clone();
                                crate::functions::alloc_type_user_data::alloc_type_user_data(
                                    l, variant, true,
                                );
                                luaur_vm::functions::lua_setfield::lua_setfield(
                                    l_vm,
                                    -2,
                                    name_c.as_ptr(),
                                );
                            }
                        }
                    } else {
                        // lua_pushlightuserdata(L, definition.first);
                        // lua_pushcclosure(L, evaluateTypeAliasCall, name.c_str(), 1);
                        // lua_setfield(L, -2, name.c_str());
                        luaur_vm::macros::lua_pushlightuserdata::lua_pushlightuserdata(
                            l as *mut c_void,
                            *def_ptr as *mut c_void,
                        );
                        luaur_vm::macros::lua_pushcclosure::lua_pushcclosure(
                            l_vm,
                            Some(evaluate_type_alias_call_thunk),
                            name_c.as_ptr(),
                            1,
                        );
                        luaur_vm::functions::lua_setfield::lua_setfield(l_vm, -2, name_c.as_ptr());
                    }
                }
            }

            // lua_setreadonly(L, -1, true);
            // lua_pop(L, 2);
            luaur_vm::functions::lua_setreadonly::lua_setreadonly(l_vm, -1, 1);
            luaur_vm::macros::lua_pop::lua_pop(l_vm, 2);
        }

        // Fetch the function we want to evaluate
        // lua_pushlightuserdata(L, typeFunction->userFuncData.definition);
        // lua_gettable(L, LUA_REGISTRYINDEX);
        luaur_vm::macros::lua_pushlightuserdata::lua_pushlightuserdata(
            l as *mut c_void,
            (*type_function).user_func_data.definition as *mut c_void,
        );
        luaur_vm::functions::lua_gettable::lua_gettable(
            l_vm,
            luaur_vm::macros::lua_registryindex::LUA_REGISTRYINDEX,
        );

        // if (!lua_isfunction(L, -1))
        if !luaur_vm::lua_isfunction!(l_vm, -1) {
            (*ctx_ptr)
                .ice
                .as_ref()
                .ice_string("user-defined type function reference cannot be found in the registry");
            return TypeFunctionReductionResult {
                result: None,
                reduction_status: Reduction::Erroneous,
                blocked_types: Vec::new(),
                blocked_packs: Vec::new(),
                error: None,
                messages: Vec::new(),
            };
        }

        // resetTypeFunctionState(L);
        reset_type_function_state(l);

        // Push serialized arguments onto the stack
        // for (auto typeParam : typeParams)
        for &type_param in type_params.iter() {
            // TypeId ty = follow(typeParam);
            let ty = follow(type_param);
            // LUAU_ASSERT(!isPending(ty, ctx->solver));
            luaur_common::macros::luau_assert::LUAU_ASSERT!(!is_pending(ty, (*ctx_ptr).solver));

            // TypeFunctionTypeId serializedTy = serialize(ty, runtimeBuilder.get());
            let serialized_ty: TypeFunctionTypeId =
                serialize_type_id_type_function_runtime_builder_state(ty, runtime_builder_ptr);

            // Check if there were any errors while serializing
            if luaur_common::FFlag::LuauTypeFunctionStructuredErrors.get() {
                if !runtime_builder.errors.is_empty() {
                    return TypeFunctionReductionResult {
                        result: None,
                        reduction_status: Reduction::Erroneous,
                        blocked_types: Vec::new(),
                        blocked_packs: Vec::new(),
                        error: Some(to_string(&runtime_builder.errors[0])),
                        messages: Vec::new(),
                    };
                }
            } else {
                if !runtime_builder.errors_deprecated.is_empty() {
                    return TypeFunctionReductionResult {
                        result: None,
                        reduction_status: Reduction::Erroneous,
                        blocked_types: Vec::new(),
                        blocked_packs: Vec::new(),
                        error: Some(runtime_builder.errors_deprecated[0].clone()),
                        messages: Vec::new(),
                    };
                }
            }

            // if (FFlag::LuauTypeFunctionRobustness && !serializedTy)
            if luaur_common::FFlag::LuauTypeFunctionRobustness.get() && serialized_ty.is_null() {
                return TypeFunctionReductionResult {
                    result: None,
                    reduction_status: Reduction::Erroneous,
                    blocked_types: Vec::new(),
                    blocked_packs: Vec::new(),
                    error: Some(
                        "Complexity limit reached when passing a type to a type function"
                            .to_string(),
                    ),
                    messages: Vec::new(),
                };
            }

            // allocTypeUserData(L, serializedTy->type);
            let variant = (*(serialized_ty as *mut TypeFunctionType))
                .type_variant
                .clone();
            crate::functions::alloc_type_user_data::alloc_type_user_data(l, variant, false);
        }

        // Set up an interrupt handler for type functions to respect type checking limits and LSP cancellation requests.
        // lua_callbacks(L)->interrupt = [](lua_State* L, int gc) { ... };
        (*luaur_vm::functions::lua_callbacks::lua_callbacks(l_vm)).interrupt =
            Some(user_defined_type_function_interrupt);

        // ctx->typeFunctionRuntime->messages.clear();
        (*type_function_runtime).messages.clear();

        // lua_pcall(L, int(typeParams.size()), 1, 0)
        let pcall_result =
            luaur_vm::functions::lua_pcall::lua_pcall(l_vm, type_params.len() as i32, 1, 0);

        let result: TypeFunctionReductionResult;

        if luaur_common::FFlag::LuauTypeFunctionStructuredErrors.get() {
            // if (auto error = checkResultForError(L, name.value, lua_pcall(...)))
            //     return {..., toString(*error), ctx->typeFunctionRuntime->messages};
            if let Some(error) = check_result_for_error(l, &name_str, pcall_result) {
                return TypeFunctionReductionResult {
                    result: None,
                    reduction_status: Reduction::Erroneous,
                    blocked_types: Vec::new(),
                    blocked_packs: Vec::new(),
                    error: Some(to_string(&error)),
                    messages: (*type_function_runtime).messages.clone(),
                };
            }
        } else {
            // if (auto error = checkResultForError_DEPRECATED(L, name.value, lua_pcall(...)))
            //     return {..., std::move(error), ctx->typeFunctionRuntime->messages};
            if let Some(error) = check_result_for_error_deprecated(l, &name_str, pcall_result) {
                return TypeFunctionReductionResult {
                    result: None,
                    reduction_status: Reduction::Erroneous,
                    blocked_types: Vec::new(),
                    blocked_packs: Vec::new(),
                    error: Some(error),
                    messages: (*type_function_runtime).messages.clone(),
                };
            }
        }

        // If the return value is not a type userdata, return with error message
        // if (!isTypeUserData(L, 1))
        if !is_type_user_data(l, 1) {
            return TypeFunctionReductionResult {
                result: None,
                reduction_status: Reduction::Erroneous,
                blocked_types: Vec::new(),
                blocked_packs: Vec::new(),
                error: Some(luaur_common::functions::format::format(format_args!(
                    "'{}' type function: returned a non-type value",
                    name_str
                ))),
                messages: (*type_function_runtime).messages.clone(),
            };
        }

        // TypeFunctionTypeId retTypeFunctionTypeId = getTypeUserData(L, 1);
        let ret_type_function_type_id: TypeFunctionTypeId = get_type_user_data(l, 1);

        if luaur_common::FFlag::LuauTypeFunctionStructuredErrors.get() {
            // No errors should be present here since we should've returned already if any were raised during serialization.
            // LUAU_ASSERT(runtimeBuilder->errors.empty());
            luaur_common::macros::luau_assert::LUAU_ASSERT!(runtime_builder.errors.is_empty());

            // TypeId retTypeId = deserialize(retTypeFunctionTypeId, runtimeBuilder.get());
            let ret_type_id = deserialize_type_function_type_id_type_function_runtime_builder_state(
                ret_type_function_type_id,
                runtime_builder_ptr,
            );

            // At least 1 error occurred while deserializing
            // if (!runtimeBuilder->errors.empty())
            if !runtime_builder.errors.is_empty() {
                result = TypeFunctionReductionResult {
                    result: None,
                    reduction_status: Reduction::Erroneous,
                    blocked_types: Vec::new(),
                    blocked_packs: Vec::new(),
                    error: Some(to_string(&runtime_builder.errors[0])),
                    messages: (*type_function_runtime).messages.clone(),
                };
            } else {
                result = TypeFunctionReductionResult {
                    result: Some(ret_type_id),
                    reduction_status: Reduction::MaybeOk,
                    blocked_types: Vec::new(),
                    blocked_packs: Vec::new(),
                    error: None,
                    messages: (*type_function_runtime).messages.clone(),
                };
            }
        } else {
            // LUAU_ASSERT(runtimeBuilder->errors_DEPRECATED.size() == 0);
            luaur_common::macros::luau_assert::LUAU_ASSERT!(runtime_builder
                .errors_deprecated
                .is_empty());

            // TypeId retTypeId = deserialize(retTypeFunctionTypeId, runtimeBuilder.get());
            let ret_type_id = deserialize_type_function_type_id_type_function_runtime_builder_state(
                ret_type_function_type_id,
                runtime_builder_ptr,
            );

            // if (runtimeBuilder->errors_DEPRECATED.size() > 0)
            if !runtime_builder.errors_deprecated.is_empty() {
                result = TypeFunctionReductionResult {
                    result: None,
                    reduction_status: Reduction::Erroneous,
                    blocked_types: Vec::new(),
                    blocked_packs: Vec::new(),
                    error: Some(runtime_builder.errors_deprecated[0].clone()),
                    messages: (*type_function_runtime).messages.clone(),
                };
            } else {
                result = TypeFunctionReductionResult {
                    result: Some(ret_type_id),
                    reduction_status: Reduction::MaybeOk,
                    blocked_types: Vec::new(),
                    blocked_packs: Vec::new(),
                    error: None,
                    messages: (*type_function_runtime).messages.clone(),
                };
            }
        }

        // C++ `LuauTempThreadPopper` pops the temp thread in its destructor. The
        // Rust port models the destructor as an explicit method (no Drop impl),
        // so invoke it here at the single success exit, mirroring scope-end RAII.
        popper.LuauTempThreadPopper();
        result
    }
}

/// Helper: read an `AstName.value` (`*const c_char`) into an owned `String`.
unsafe fn ast_name_to_string(value: *const core::ffi::c_char) -> alloc::string::String {
    if value.is_null() {
        alloc::string::String::new()
    } else {
        core::ffi::CStr::from_ptr(value)
            .to_string_lossy()
            .into_owned()
    }
}

/// Helper: `ctx->builtins->errorType`.
unsafe fn error_type(ctx_ptr: *mut TypeFunctionContext) -> TypeId {
    (*(*ctx_ptr).builtins.as_ptr()).errorType
}

/// `lua_CFunction` thunk for `evaluateTypeAliasCall`. The closure is registered
/// via `lua_pushcclosure`, which expects a `lua_CFunction`
/// (`Option<unsafe fn(*mut lua_State) -> c_int>`, Rust ABI).
unsafe fn evaluate_type_alias_call_thunk(
    l: *mut luaur_vm::records::lua_state::lua_State,
) -> core::ffi::c_int {
    crate::functions::evaluate_type_alias_call::evaluate_type_alias_call(l as *mut lua_State)
}
