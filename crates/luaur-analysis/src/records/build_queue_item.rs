use crate::records::frontend::FrontendStats;
use crate::records::frontend_options::FrontendOptions;
use crate::records::internal_compiler_error::InternalCompilerError;
use crate::records::require_cycle::RequireCycle;
use crate::records::source_module::SourceModule;
use crate::records::source_node::SourceNode;
use crate::type_aliases::module_name_file_resolver::ModuleName;
use crate::type_aliases::module_ptr_module_resolver::ModulePtr;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use alloc::sync::Arc;
use alloc::vec::Vec;
use luaur_config::records::config::Config;

#[derive(Debug, Clone)]
pub struct BuildQueueItem {
    pub name: ModuleName,
    pub human_readable_name: ModuleName,
    pub source_node: Arc<SourceNode>,
    pub source_module: Arc<SourceModule>,
    pub config: Config,
    pub environment_scope: ScopePtr,
    pub require_cycles: Vec<RequireCycle>,
    pub options: FrontendOptions,
    pub record_json_log: bool,
    pub reverse_deps: Vec<usize>,
    pub dirty_dependencies: i32,
    pub processing: bool,
    // Result
    // C++: `std::exception_ptr exception;` which here only ever holds a
    // `Luau::InternalCompilerError` (a recursion/internal compiler error). It is
    // used as a presence flag (`if (item.exception)`) and later rethrown in
    // `recordItemResult`. Modeled as an `Option` carrying the caught error.
    pub exception: Option<InternalCompilerError>,
    pub module: ModulePtr,
    pub stats: FrontendStats,
}
