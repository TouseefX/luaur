use crate::records::fixture::Fixture;
use luaur_analysis::enums::solver_mode::SolverMode;
use luaur_analysis::functions::freeze::freeze;
use luaur_analysis::records::frontend::Frontend;
use luaur_analysis::records::frontend_options::FrontendOptions;
use luaur_ast::enums::mode::Mode;

impl Fixture {
    pub fn get_frontend(&mut self) -> &mut Frontend {
        let newly_initialized = self.frontend.is_none();

        if newly_initialized {
            let mode = if luaur_common::FFlag::DebugLuauForceOldSolver.get() {
                SolverMode::Old
            } else {
                SolverMode::New
            };

            let mut options = FrontendOptions::default();
            options.retain_full_type_graphs = true;
            options.for_autocomplete = false;
            options.run_lint_checks = false;

            let frontend =
                Frontend::frontend_solver_mode_file_resolver_config_resolver_frontend_options(
                    mode,
                    &mut self.file_resolver.base,
                    &mut self.config_resolver.base,
                    options,
                );

            self.frontend = Some(frontend);
            self.config_resolver.default_config.mode = Mode::Strict;
            self.config_resolver
                .default_config
                .enabled_lint
                .warning_mask = !0u64;
            self.config_resolver
                .default_config
                .parse_options
                .capture_comments = true;
        }

        let file_resolver = &mut self.file_resolver.base;
        let config_resolver = &mut self.config_resolver.base;
        let frontend = self.frontend.as_mut().unwrap();

        frontend.file_resolver = file_resolver;
        frontend.config_resolver = config_resolver;
        unsafe {
            frontend.wire_self_pointers();
        }

        if newly_initialized {
            freeze(frontend.globals.global_types_mut());
            freeze(frontend.globals_for_autocomplete.global_types_mut());
        }

        self.builtin_types = frontend.builtin_types;
        frontend
    }
}
