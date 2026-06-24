//! Source: `Analysis/include/Luau/Frontend.h` (hand-ported; fields only)

use crate::records::builtin_types::BuiltinTypes;
use crate::records::config_resolver::ConfigResolver;
use crate::records::file_resolver::FileResolver;
use crate::records::frontend_module_resolver::FrontendModuleResolver;
use crate::records::frontend_options::FrontendOptions;
use crate::records::global_types::GlobalTypes;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::require_trace_result::RequireTraceResult;
use crate::records::source_module::SourceModule;
use crate::records::source_node::SourceNode;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::AtomicI32;
use luaur_config::type_aliases::module_name::ModuleName;
use std::collections::HashMap;

/// Frontend::Stats (nested struct)
#[derive(Debug, Clone, Copy, Default)]
pub struct FrontendStats {
    pub files: usize,
    pub lines: usize,
    pub files_strict: usize,
    pub files_nonstrict: usize,
    pub types_allocated: usize,
    pub type_packs_allocated: usize,
    pub bool_singletons_minted: usize,
    pub str_singletons_minted: usize,
    pub unique_str_singletons_minted: usize,
    pub time_read: f64,
    pub time_parse: f64,
    pub time_check: f64,
    pub time_lint: f64,
    pub dynamic_constraints_created: usize,
}

pub struct Frontend {
    pub use_new_luau_solver: AtomicI32,

    pub environments: HashMap<String, ScopePtr>,
    pub builtin_definitions: HashMap<String, Rc<dyn Fn(&mut Frontend, &mut GlobalTypes, ScopePtr)>>,

    pub builtin_types_: BuiltinTypes,
    pub builtin_types: *mut BuiltinTypes, // NotNull, points at builtin_types_

    pub file_resolver: *mut FileResolver,
    pub module_resolver: FrontendModuleResolver,
    pub module_resolver_for_autocomplete: FrontendModuleResolver,
    pub globals: GlobalTypes,
    pub globals_for_autocomplete: GlobalTypes,
    pub config_resolver: *mut ConfigResolver,
    pub options: FrontendOptions,
    pub ice_handler: InternalErrorReporter,
    pub prepare_module_scope: Option<Rc<dyn Fn(&ModuleName, &ScopePtr, bool)>>,
    pub write_json_log: Option<Rc<dyn Fn(&ModuleName, String)>>,

    pub source_nodes: HashMap<ModuleName, Arc<SourceNode>>,
    pub source_modules: HashMap<ModuleName, Arc<SourceModule>>,
    pub require_trace: HashMap<ModuleName, RequireTraceResult>,

    pub stats: FrontendStats,

    pub module_queue: Vec<ModuleName>,
}

impl core::fmt::Debug for Frontend {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Frontend")
            .field("use_new_luau_solver", &self.use_new_luau_solver)
            .field("environments", &self.environments)
            .field("builtin_definitions_len", &self.builtin_definitions.len())
            .field("builtin_types_", &self.builtin_types_)
            .field("builtin_types", &self.builtin_types)
            .field("file_resolver", &self.file_resolver)
            .field("module_resolver", &self.module_resolver)
            .field(
                "module_resolver_for_autocomplete",
                &self.module_resolver_for_autocomplete,
            )
            .field("globals", &self.globals)
            .field("globals_for_autocomplete", &self.globals_for_autocomplete)
            .field("config_resolver", &self.config_resolver)
            .field("options", &self.options)
            .field("ice_handler", &self.ice_handler)
            .field(
                "prepare_module_scope",
                &self.prepare_module_scope.as_ref().map(|_| "..."),
            )
            .field(
                "write_json_log",
                &self.write_json_log.as_ref().map(|_| "..."),
            )
            .field("source_nodes", &self.source_nodes)
            .field("source_modules", &self.source_modules)
            .field("require_trace", &self.require_trace)
            .field("stats", &self.stats)
            .field("module_queue", &self.module_queue)
            .finish()
    }
}
