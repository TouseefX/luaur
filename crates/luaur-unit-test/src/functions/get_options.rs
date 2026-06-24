use luaur_analysis::records::frontend_options::FrontendOptions;

pub fn get_options() -> FrontendOptions {
    let mut options = FrontendOptions {
        retain_full_type_graphs: true,
        for_autocomplete: false,
        run_lint_checks: false,
        randomize_constraint_resolution_seed: None,
        enabled_lint_warnings: None,
        cancellation_token: None,
        module_time_limit_sec: None,
        apply_internal_limit_scaling: false,
        custom_module_check: None,
        collect_type_allocation_stats: false,
    };

    if luaur_common::FFlag::DebugLuauForceOldSolver.get() {
        options.for_autocomplete = true;
    }

    options
}
