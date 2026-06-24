use crate::enums::solver_mode::SolverMode;
use crate::functions::apply_internal_limit_scaling::apply_internal_limit_scaling;
use crate::functions::copy_errors::copy_errors;
use crate::functions::filter_lint_options::filter_lint_options;
use crate::functions::freeze::freeze;
use crate::functions::get_timestamp::get_timestamp;
use crate::functions::lint::lint;
use crate::functions::make_type_check_limits::make_type_check_limits;
use crate::functions::unfreeze::unfreeze;
use crate::records::build_queue_item::BuildQueueItem;
use crate::records::frontend::Frontend;
use crate::records::module::Module;
use crate::records::module_has_cyclic_dependency::ModuleHasCyclicDependency;
use crate::records::source_node::SourceNode;
use crate::records::syntax_error::SyntaxError;
use crate::records::type_error::TypeError;
use crate::type_aliases::type_error_data::TypeErrorData;
use alloc::sync::Arc;
use alloc::vec::Vec;
use luaur_ast::enums::mode::Mode;
use luaur_common::macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE;
use luaur_common::FFlag;
use luaur_common::FInt;

impl Frontend {
    pub fn check_build_queue_item(&mut self, item: &mut BuildQueueItem) {
        // SAFETY: `item.source_node` / `item.source_module` are `Arc`s owned by the queue.
        // The C++ takes them by reference and mutates `sourceModule.mode` in place; we mirror
        // that with raw pointers through the `Arc`.
        let source_node_ptr = Arc::as_ptr(&item.source_node) as *mut SourceNode;

        let mode: Mode = if FFlag::DebugLuauForceStrictMode.get() {
            Mode::Strict
        } else if FFlag::DebugLuauForceNonStrictMode.get() {
            Mode::Nonstrict
        } else {
            item.source_module.mode.unwrap_or(item.config.mode)
        };

        unsafe {
            let source_module_ptr = Arc::as_ptr(&item.source_module)
                as *mut crate::records::source_module::SourceModule;
            (*source_module_ptr).mode = Some(mode);
        }

        let environment_scope = item.environment_scope.clone();
        let timestamp = get_timestamp();
        let require_cycles = item.require_cycles.clone();

        let mut type_check_limits = make_type_check_limits(&item.options);

        // TODO (C++ comment retained): dirty ad hoc solution for autocomplete timeouts.
        if item.options.apply_internal_limit_scaling {
            let autocomplete_mult = unsafe { (*source_node_ptr).autocomplete_limits_mult };

            if FInt::LuauTarjanChildLimit.get() > 0 {
                type_check_limits.instantiationChildLimit = Some(
                    1.max((FInt::LuauTarjanChildLimit.get() as f64 * autocomplete_mult) as i32),
                );
            } else {
                type_check_limits.instantiationChildLimit = None;
            }

            if FInt::LuauTypeInferIterationLimit.get() > 0 {
                type_check_limits.unifierIterationLimit = Some(1.max(
                    (FInt::LuauTypeInferIterationLimit.get() as f64 * autocomplete_mult) as i32,
                ));
            } else {
                type_check_limits.unifierIterationLimit = None;
            }
        }

        if item.options.for_autocomplete {
            // The autocomplete typecheck is always in strict mode with DM awareness.
            let module_for_autocomplete = self.check_source_module_mode_vector_require_cycle_optional_scope_ptr_bool_bool_frontend_stats_type_check_limits(
                &item.source_module,
                Mode::Strict,
                require_cycles,
                Some(environment_scope.clone()),
                /* for_autocomplete */ true,
                /* record_json_log */ false,
                &mut item.stats,
                type_check_limits,
            );

            let duration = get_timestamp() - timestamp;
            let module_ptr = Arc::as_ptr(&module_for_autocomplete) as *mut Module;
            unsafe {
                (*module_ptr).check_duration_sec = duration;
            }

            self.populate_expected_types(&item.source_module, module_ptr, &environment_scope);

            if item.options.module_time_limit_sec.is_some()
                && item.options.apply_internal_limit_scaling
            {
                apply_internal_limit_scaling(
                    unsafe { &mut *source_node_ptr },
                    module_for_autocomplete.clone(),
                    item.options.module_time_limit_sec.unwrap(),
                );
            }

            item.stats.time_check += duration;
            item.stats.files_strict += 1;

            if item.options.collect_type_allocation_stats {
                let m = &*module_for_autocomplete;
                item.stats.types_allocated += m.internal_types.types.size();
                item.stats.type_packs_allocated += m.internal_types.type_packs.size();
                item.stats.bool_singletons_minted += m.internal_types.bool_singletons_minted;
                item.stats.str_singletons_minted += m.internal_types.str_singletons_minted;
                item.stats.unique_str_singletons_minted +=
                    m.internal_types.unique_str_singletons_minted.size();
            }

            if let Some(custom_module_check) = item.options.custom_module_check {
                custom_module_check(&item.source_module, &module_for_autocomplete);
            }

            item.module = module_for_autocomplete;
            return;
        }

        let module = self.check_source_module_mode_vector_require_cycle_optional_scope_ptr_bool_bool_frontend_stats_type_check_limits(
            &item.source_module,
            mode,
            require_cycles.clone(),
            Some(environment_scope.clone()),
            /* for_autocomplete */ false,
            item.record_json_log,
            &mut item.stats,
            type_check_limits,
        );

        let duration = get_timestamp() - timestamp;
        let module_ptr: *mut Module = Arc::as_ptr(&module) as *mut Module;
        unsafe {
            (*module_ptr).check_duration_sec = duration;
        }

        self.populate_expected_types(&item.source_module, module_ptr, &environment_scope);

        if item.options.module_time_limit_sec.is_some() && item.options.apply_internal_limit_scaling
        {
            apply_internal_limit_scaling(
                unsafe { &mut *source_node_ptr },
                module.clone(),
                item.options.module_time_limit_sec.unwrap(),
            );
        }

        item.stats.time_check += duration;
        item.stats.files_strict += if mode == Mode::Strict { 1 } else { 0 };
        item.stats.files_nonstrict += if mode == Mode::Nonstrict { 1 } else { 0 };

        if item.options.collect_type_allocation_stats {
            let m = &*module;
            item.stats.types_allocated += m.internal_types.types.size();
            item.stats.type_packs_allocated += m.internal_types.type_packs.size();
            item.stats.bool_singletons_minted += m.internal_types.bool_singletons_minted;
            item.stats.str_singletons_minted += m.internal_types.str_singletons_minted;
            item.stats.unique_str_singletons_minted +=
                m.internal_types.unique_str_singletons_minted.size();
        }

        if let Some(custom_module_check) = item.options.custom_module_check {
            custom_module_check(&item.source_module, &module);
        }

        if self.get_luau_solver_mode() == SolverMode::New && mode == Mode::NoCheck {
            unsafe {
                (*module_ptr).errors.clear();
            }
        }

        if item.options.run_lint_checks {
            LUAU_TIMETRACE_SCOPE!("lint", "Frontend");

            let mut lint_options = item
                .options
                .enabled_lint_warnings
                .clone()
                .unwrap_or(item.config.enabled_lint);
            filter_lint_options(&mut lint_options, &item.source_module.hotcomments, mode);

            let lint_timestamp = get_timestamp();

            let warnings = lint(
                item.source_module.root as *mut luaur_ast::records::ast_stat::AstStat,
                item.source_module.names.as_ref(),
                &environment_scope,
                module_ptr as *const Module,
                &item.source_module.hotcomments,
                &lint_options,
            );

            item.stats.time_lint += get_timestamp() - lint_timestamp;

            let lint_result = self.classify_lints(&warnings, &item.config);
            unsafe {
                (*module_ptr).lint_result = lint_result;
            }
        }

        if !item.options.retain_full_type_graphs {
            // copyErrors needs to allocate into interfaceTypes as it copies types out of
            // internalTypes, so we unfreeze it here.
            unsafe {
                unfreeze(&mut (*module_ptr).interface_types);
                // copyErrors(module->errors, module->interfaceTypes, builtinTypes);
                let mut errors = core::mem::take(&mut (*module_ptr).errors);
                copy_errors(
                    &mut errors,
                    &mut (*module_ptr).interface_types,
                    &*self.builtin_types,
                );
                (*module_ptr).errors = errors;
                freeze(&mut (*module_ptr).interface_types);

                (*module_ptr).internal_types.clear();
                (*module_ptr).def_arena.allocator.clear();
                (*module_ptr).key_arena.allocator.clear();

                (*module_ptr).ast_types.clear();
                (*module_ptr).ast_type_packs.clear();
                (*module_ptr).ast_expected_types.clear();
                (*module_ptr).ast_original_call_types.clear();
                (*module_ptr).ast_overload_resolved_types.clear();
                (*module_ptr).ast_for_in_next_types.clear();
                (*module_ptr).ast_resolved_types.clear();
                (*module_ptr).ast_resolved_type_packs.clear();
                (*module_ptr).ast_compound_assign_result_types.clear();
                (*module_ptr).ast_scopes.clear();
                (*module_ptr).upper_bound_contributors.clear();
                (*module_ptr).scopes.clear();
            }
        }

        if mode != Mode::NoCheck {
            for cyc in &require_cycles {
                let te = TypeError {
                    location: cyc.location,
                    module_name: item.name.clone(),
                    data: TypeErrorData::ModuleHasCyclicDependency(ModuleHasCyclicDependency::new(
                        cyc.path.clone(),
                    )),
                };
                unsafe {
                    (*module_ptr).errors.push(te);
                }
            }
        }

        let mut parse_errors: Vec<TypeError> = Vec::new();
        for pe in &item.source_module.parse_errors {
            parse_errors.push(TypeError {
                location: *pe.get_location(),
                module_name: item.name.clone(),
                data: TypeErrorData::SyntaxError(SyntaxError::new(alloc::string::String::from(
                    pe.what(),
                ))),
            });
        }
        unsafe {
            // module->errors.insert(module->errors.begin(), parseErrors.begin(), parseErrors.end());
            let mut combined = parse_errors;
            combined.append(&mut (*module_ptr).errors);
            (*module_ptr).errors = combined;
        }

        item.module = module;
    }
}
