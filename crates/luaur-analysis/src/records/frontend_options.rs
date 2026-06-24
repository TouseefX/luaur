use alloc::sync::Arc;
use luaur_ast::records::location::Location;
use luaur_config::records::lint_options::LintOptions;

use crate::records::frontend_cancellation_token::FrontendCancellationToken;
use crate::records::module::Module;
use crate::records::source_module::SourceModule;

/// Port of `Luau::FrontendOptions` from `Analysis/include/Luau/Frontend.h`.
#[derive(Debug, Clone)]
pub struct FrontendOptions {
    pub retain_full_type_graphs: bool,
    pub for_autocomplete: bool,
    pub run_lint_checks: bool,
    pub randomize_constraint_resolution_seed: Option<u32>,
    pub enabled_lint_warnings: Option<LintOptions>,
    pub cancellation_token: Option<Arc<FrontendCancellationToken>>,
    pub module_time_limit_sec: Option<f64>,
    pub apply_internal_limit_scaling: bool,
    pub custom_module_check: Option<fn(&SourceModule, &Module)>,
    pub collect_type_allocation_stats: bool,
}

impl Default for FrontendOptions {
    fn default() -> Self {
        Self {
            retain_full_type_graphs: false,
            for_autocomplete: false,
            run_lint_checks: false,
            randomize_constraint_resolution_seed: None,
            enabled_lint_warnings: None,
            cancellation_token: None,
            module_time_limit_sec: None,
            apply_internal_limit_scaling: false,
            custom_module_check: None,
            collect_type_allocation_stats: false,
        }
    }
}
