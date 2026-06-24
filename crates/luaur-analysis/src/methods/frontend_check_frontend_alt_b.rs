use crate::enums::solver_mode::SolverMode;
use crate::functions::check_frontend::check as check_new_solver;
use crate::records::frontend::{Frontend, FrontendStats};
use crate::records::require_cycle::RequireCycle;
use crate::records::source_module::SourceModule;
use crate::records::stats::Stats;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::module_ptr_module::ModulePtr;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use alloc::rc::Rc;
use alloc::vec::Vec;
use luaur_ast::enums::mode::Mode;

impl Frontend {
    pub fn check_source_module_mode_vector_require_cycle_optional_scope_ptr_bool_bool_frontend_stats_type_check_limits(
        &mut self,
        source_module: &SourceModule,
        mode: Mode,
        require_cycles: Vec<RequireCycle>,
        environment_scope: Option<ScopePtr>,
        for_autocomplete: bool,
        record_json_log: bool,
        stats: &mut FrontendStats,
        type_check_limits: TypeCheckLimits,
    ) -> ModulePtr {
        if self.get_luau_solver_mode() == SolverMode::New {
            let prepare_module_scope_wrap = {
                let prepare_module_scope = self.prepare_module_scope.clone();
                Rc::new(
                    move |name: &crate::type_aliases::module_name_type::ModuleName,
                          scope: &ScopePtr| {
                        if let Some(prepare_module_scope) = &prepare_module_scope {
                            prepare_module_scope(name, scope, for_autocomplete);
                        }
                    },
                )
            };

            let mut function_stats = Stats {
                files: stats.files,
                lines: stats.lines,
                files_strict: stats.files_strict,
                files_nonstrict: stats.files_nonstrict,
                types_allocated: stats.types_allocated,
                type_packs_allocated: stats.type_packs_allocated,
                bool_singletons_minted: stats.bool_singletons_minted,
                str_singletons_minted: stats.str_singletons_minted,
                unique_str_singletons_minted: stats.unique_str_singletons_minted,
                time_read: stats.time_read,
                time_parse: stats.time_parse,
                time_check: stats.time_check,
                time_lint: stats.time_lint,
                dynamic_constraints_created: stats.dynamic_constraints_created,
            };

            let write_json_log = self.write_json_log.clone().unwrap_or_else(|| {
                Rc::new(
                    |_: &crate::type_aliases::module_name_type::ModuleName,
                     _: alloc::string::String| {},
                )
            });

            let module = check_new_solver(
                source_module,
                mode,
                &require_cycles,
                self.builtin_types,
                &mut self.ice_handler,
                if for_autocomplete {
                    &mut self.module_resolver_for_autocomplete as *mut _
                        as *mut crate::records::module_resolver::ModuleResolver
                } else {
                    &mut self.module_resolver as *mut _
                        as *mut crate::records::module_resolver::ModuleResolver
                },
                self.file_resolver,
                environment_scope
                    .as_ref()
                    .unwrap_or(&self.globals.global_scope),
                &self.globals.global_type_function_scope,
                prepare_module_scope_wrap,
                self.options.clone(),
                type_check_limits,
                record_json_log,
                &mut function_stats,
                write_json_log,
            );

            stats.files = function_stats.files;
            stats.lines = function_stats.lines;
            stats.files_strict = function_stats.files_strict;
            stats.files_nonstrict = function_stats.files_nonstrict;
            stats.types_allocated = function_stats.types_allocated;
            stats.type_packs_allocated = function_stats.type_packs_allocated;
            stats.bool_singletons_minted = function_stats.bool_singletons_minted;
            stats.str_singletons_minted = function_stats.str_singletons_minted;
            stats.unique_str_singletons_minted = function_stats.unique_str_singletons_minted;
            stats.time_read = function_stats.time_read;
            stats.time_parse = function_stats.time_parse;
            stats.time_check = function_stats.time_check;
            stats.time_lint = function_stats.time_lint;
            stats.dynamic_constraints_created = function_stats.dynamic_constraints_created;

            module
        } else {
            let global_scope = if for_autocomplete {
                &self.globals_for_autocomplete.global_scope
            } else {
                &self.globals.global_scope
            };
            let resolver = if for_autocomplete {
                &mut self.module_resolver_for_autocomplete as *mut _
                    as *mut crate::records::module_resolver::ModuleResolver
            } else {
                &mut self.module_resolver as *mut _
                    as *mut crate::records::module_resolver::ModuleResolver
            };
            let mut type_checker = TypeChecker::new(
                global_scope,
                resolver,
                self.builtin_types,
                &mut self.ice_handler,
            );
            if self.prepare_module_scope.is_some() {
                let prepare_module_scope = self.prepare_module_scope.clone();
                type_checker.prepare_module_scope = Some(Rc::new(move |name, scope| {
                    if let Some(prepare_module_scope) = &prepare_module_scope {
                        prepare_module_scope(name, scope, for_autocomplete);
                    }
                }));
            }
            type_checker.require_cycles = require_cycles;
            type_checker.finish_time = type_check_limits.finishTime();
            type_checker.instantiation_child_limit = type_check_limits.instantiationChildLimit();
            type_checker.unifier_iteration_limit = type_check_limits.unifierIterationLimit();
            type_checker.cancellation_token = type_check_limits.cancellationToken();

            type_checker.check_source_module_mode_optional_scope_ptr(
                source_module,
                mode,
                environment_scope,
            )
        }
    }
}
