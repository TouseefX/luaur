//! Node: `cxx:Function:Luau.Analysis:Analysis/src/UserDefinedTypeFunction.cpp:99:evaluateTypeAliasCall`
//! Source: `Analysis/src/UserDefinedTypeFunction.cpp:99-217`
//!
//! Faithful port of `static int evaluateTypeAliasCall(lua_State* L)`. This is the
//! C closure body registered (via `lua_pushcclosure`) for each parameterised
//! type alias visible to a user-defined type function: it deserializes the Lua
//! type arguments, saturates them against the alias' declared parameters,
//! instantiates the alias body, reduces any type functions inside it, then
//! serializes the result back into a (frozen) type userdata.
use crate::functions::deserialize_type_function_runtime_builder::deserialize_type_function_type_id_type_function_runtime_builder_state;
use crate::functions::follow_type::follow;
use crate::functions::get_type_function_runtime::get_type_function_runtime;
use crate::functions::get_type_user_data::get_type_user_data;
use crate::functions::reduce_type_functions_type_function::reduce_type_functions;
use crate::functions::saturate_arguments::saturate_arguments;
use crate::functions::serialize_type_function_runtime_builder::serialize_type_id_type_function_runtime_builder_state;
use crate::functions::to_string_type_function_error::to_string;
use crate::records::apply_type_function::ApplyTypeFunction;
use crate::records::freeze_type_function_types::FreezeTypeFunctionTypes;
use crate::records::substitution::Substitution;
use crate::records::type_fun::TypeFun;
use crate::records::type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState;
use crate::records::type_function_type::TypeFunctionType;
use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;
use luaur_ast::records::location::Location;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[allow(non_snake_case)]
pub fn evaluate_type_alias_call(L: *mut lua_State) -> i32 {
    unsafe {
        // TypeFun* tf = static_cast<TypeFun*>(lua_tolightuserdata(L, lua_upvalueindex(1)));
        let tf = luaur_vm::functions::lua_tolightuserdata::lua_tolightuserdata(
            L as *mut luaur_vm::records::lua_state::lua_State,
            luaur_vm::macros::lua_upvalueindex::lua_upvalueindex(1),
        ) as *mut TypeFun;

        // TypeFunctionRuntime* runtime = getTypeFunctionRuntime(L);
        // TypeFunctionRuntimeBuilderState* runtimeBuilder = runtime->runtimeBuilder;
        let runtime = get_type_function_runtime(L);
        let runtime_builder: *mut TypeFunctionRuntimeBuilderState = (*runtime).runtime_builder;

        // ApplyTypeFunction applyTypeFunction{runtimeBuilder->ctx->arena};
        let ctx = (*runtime_builder).ctx;
        let arena_ptr = (*ctx).arena.as_ptr();
        let mut apply_type_function = ApplyTypeFunction {
            base: Substitution::substitution_new(
                crate::records::txn_log::TxnLog::empty(),
                arena_ptr,
            ),
            encountered_forwarded_type: false,
            type_arguments: DenseHashMap::new(core::ptr::null()),
            type_pack_arguments: DenseHashMap::new(core::ptr::null()),
        };

        // int argumentCount = lua_gettop(L);
        // std::vector<TypeId> rawTypeArguments;
        let argument_count = luaur_vm::functions::lua_gettop::lua_gettop(
            L as *mut luaur_vm::records::lua_state::lua_State,
        );
        let mut raw_type_arguments: Vec<TypeId> = Vec::new();

        for i in 0..argument_count {
            // TypeFunctionTypeId tfty = getTypeUserData(L, i + 1);
            let tfty: TypeFunctionTypeId = get_type_user_data(L, i + 1);
            // TypeId ty = deserialize(tfty, runtimeBuilder);
            let ty = deserialize_type_function_type_id_type_function_runtime_builder_state(
                tfty,
                runtime_builder,
            );

            // if (... ? !runtimeBuilder->errors.empty() : !runtimeBuilder->errors_DEPRECATED.empty())
            //     luaL_error(L, "failed to deserialize type at argument %d", i + 1);
            let has_errors = if luaur_common::FFlag::LuauTypeFunctionStructuredErrors.get() {
                !(*runtime_builder).errors.is_empty()
            } else {
                !(*runtime_builder).errors_deprecated.is_empty()
            };
            if has_errors {
                luaur_vm::luaL_error!(
                    L as *mut luaur_vm::records::lua_state::lua_State,
                    "failed to deserialize type at argument {}",
                    i + 1
                );
                unreachable!();
            }

            // rawTypeArguments.push_back(ty);
            raw_type_arguments.push(ty);
        }

        // Check if we have enough arguments, by typical typechecking rules
        // size_t typesRequired = tf->typeParams.size();
        // size_t packsRequired = tf->typePackParams.size();
        let types_required = (*tf).type_params().len();
        let packs_required = (*tf).type_pack_params().len();

        // size_t typesProvided = rawTypeArguments.size() > typesRequired ? typesRequired : rawTypeArguments.size();
        let mut types_provided = if raw_type_arguments.len() > types_required {
            types_required
        } else {
            raw_type_arguments.len()
        };
        // size_t extraTypes = rawTypeArguments.size() > typesRequired ? rawTypeArguments.size() - typesRequired : 0;
        let extra_types = if raw_type_arguments.len() > types_required {
            raw_type_arguments.len() - types_required
        } else {
            0
        };
        // size_t packsProvided = 0;
        let mut packs_provided: usize = 0;

        // if (extraTypes != 0 && packsProvided == 0)
        if extra_types != 0 && packs_provided == 0 {
            // Extra types are only collected into a pack if a pack is expected
            if packs_required != 0 {
                packs_provided += 1;
            } else {
                types_provided += extra_types;
            }
        }

        // for (size_t i = typesProvided; i < typesRequired; ++i)
        //     if (tf->typeParams[i].defaultValue) typesProvided += 1;
        for i in types_provided..types_required {
            if (*tf).type_params()[i].defaultValue.is_some() {
                types_provided += 1;
            }
        }

        // for (size_t i = packsProvided; i < packsRequired; ++i)
        //     if (tf->typePackParams[i].defaultValue) packsProvided += 1;
        for i in packs_provided..packs_required {
            if (*tf).type_pack_params()[i].defaultValue.is_some() {
                packs_provided += 1;
            }
        }

        // if (extraTypes == 0 && packsProvided + 1 == packsRequired)
        //     packsProvided += 1;
        if extra_types == 0 && packs_provided + 1 == packs_required {
            packs_provided += 1;
        }

        // if (typesProvided != typesRequired || packsProvided != packsRequired)
        //     luaL_error(L, "not enough arguments to call");
        if types_provided != types_required || packs_provided != packs_required {
            luaur_vm::luaL_error!(
                L as *mut luaur_vm::records::lua_state::lua_State,
                "not enough arguments to call"
            );
            unreachable!();
        }

        // Prepare final types and packs
        // auto [types, packs] = saturateArguments(runtimeBuilder->ctx->arena, runtimeBuilder->ctx->builtins, *tf, rawTypeArguments, {});
        let arena_ref = &mut *(*ctx).arena.as_ptr();
        let builtins_ref = &mut *(*ctx).builtins.as_ptr();
        let empty_packs: Vec<crate::type_aliases::type_pack_id::TypePackId> = Vec::new();
        let (types, packs) = saturate_arguments(
            arena_ref,
            builtins_ref,
            &*tf,
            &raw_type_arguments,
            &empty_packs,
        );

        // for (size_t i = 0; i < types.size(); ++i)
        //     applyTypeFunction.typeArguments[tf->typeParams[i].ty] = types[i];
        for i in 0..types.len() {
            let key = (*tf).type_params()[i].ty;
            *apply_type_function.type_arguments.get_or_insert(key) = types[i];
        }

        // for (size_t i = 0; i < packs.size(); ++i)
        //     applyTypeFunction.typePackArguments[tf->typePackParams[i].tp] = packs[i];
        for i in 0..packs.len() {
            let key = (*tf).type_pack_params()[i].tp;
            *apply_type_function.type_pack_arguments.get_or_insert(key) = packs[i];
        }

        // std::optional<TypeId> maybeInstantiated = applyTypeFunction.substitute(tf->type);
        let maybe_instantiated = apply_type_function.substitute_type_id((*tf).r#type());

        // if (!maybeInstantiated.has_value())
        // {
        //     luaL_error(L, "failed to instantiate type alias");
        //     return 1;
        // }
        let instantiated = match maybe_instantiated {
            Some(v) => v,
            None => {
                luaur_vm::luaL_error!(
                    L as *mut luaur_vm::records::lua_state::lua_State,
                    "failed to instantiate type alias"
                );
                return 1;
            }
        };

        // TypeId target = follow(*maybeInstantiated);
        let target = follow(instantiated);

        // FunctionGraphReductionResult result = reduceTypeFunctions(target, Location{}, runtimeBuilder->ctx);
        let result = reduce_type_functions(
            target,
            Location::default(),
            core::ptr::NonNull::new_unchecked(ctx),
            false,
        );

        // if (!result.errors.empty())
        //     luaL_error(L, "failed to reduce type function with: %s", toString(result.errors.front()).c_str());
        if !result.errors.is_empty() {
            luaur_vm::luaL_error!(
                L as *mut luaur_vm::records::lua_state::lua_State,
                "failed to reduce type function with: {}",
                crate::functions::to_string_error::to_string_type_error(&result.errors[0])
            );
            unreachable!();
        }

        // TypeFunctionTypeId serializedTy = serialize(follow(target), runtimeBuilder);
        let mut serialized_ty: TypeFunctionTypeId =
            serialize_type_id_type_function_runtime_builder_state(follow(target), runtime_builder);

        // if (!FFlag::LuauTypeFunctionRobustness)
        if !luaur_common::FFlag::LuauTypeFunctionRobustness.get() {
            if luaur_common::FFlag::LuauTypeFunctionSupportsFrozen.get() {
                // FreezeTypeFunctionTypes freezer{}; freezer.run(serializedTy);
                let mut freezer = FreezeTypeFunctionTypes::new();
                freezer.base.run_type_function_type_id(serialized_ty);
            }
        }

        // if (FFlag::LuauTypeFunctionStructuredErrors)
        if luaur_common::FFlag::LuauTypeFunctionStructuredErrors.get() {
            // if (!runtimeBuilder->errors.empty())
            //     luaL_error(L, "%s", toString(runtimeBuilder->errors.front()).c_str());
            let errors = &(*runtime_builder).errors;
            if !errors.is_empty() {
                luaur_vm::luaL_error!(
                    L as *mut luaur_vm::records::lua_state::lua_State,
                    "{}",
                    to_string(&errors[0])
                );
                unreachable!();
            }
        } else {
            // if (!runtimeBuilder->errors_DEPRECATED.empty())
            //     luaL_error(L, "%s", runtimeBuilder->errors_DEPRECATED.front().c_str());
            let errors_deprecated = &(*runtime_builder).errors_deprecated;
            if !errors_deprecated.is_empty() {
                luaur_vm::luaL_error!(
                    L as *mut luaur_vm::records::lua_state::lua_State,
                    "{}",
                    errors_deprecated[0]
                );
                unreachable!();
            }
        }

        // if (FFlag::LuauTypeFunctionRobustness)
        if luaur_common::FFlag::LuauTypeFunctionRobustness.get() {
            // if (!serializedTy) luaL_error(L, "Complexity limit reached when passing a type to a type alias");
            if serialized_ty.is_null() {
                luaur_vm::luaL_error!(
                    L as *mut luaur_vm::records::lua_state::lua_State,
                    "Complexity limit reached when passing a type to a type alias"
                );
                unreachable!();
            }

            if luaur_common::FFlag::LuauTypeFunctionSupportsFrozen.get() {
                // FreezeTypeFunctionTypes freezer{}; freezer.run(serializedTy);
                let mut freezer = FreezeTypeFunctionTypes::new();
                freezer.base.run_type_function_type_id(serialized_ty);
            }
        }

        // Silence unused-mut when serialized_ty is only re-read.
        let _ = &mut serialized_ty;

        // allocTypeUserData(L, serializedTy->type, /* frozen */ true);
        let type_variant = (*(serialized_ty as *mut TypeFunctionType))
            .type_variant
            .clone();
        crate::functions::alloc_type_user_data::alloc_type_user_data(L, type_variant, true);

        // return 1;
        1
    }
}
