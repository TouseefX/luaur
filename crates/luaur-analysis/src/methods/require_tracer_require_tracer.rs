use crate::records::file_resolver::FileResolver;
use crate::records::require_trace_result::RequireTraceResult;
use crate::records::require_tracer::RequireTracer;
use crate::type_aliases::module_name_type::ModuleName;
use alloc::vec::Vec;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl RequireTracer {
    pub fn require_tracer(
        result: *mut RequireTraceResult,
        file_resolver: *mut FileResolver,
        current_module_name: ModuleName,
    ) -> Self {
        RequireTracer {
            result,
            file_resolver,
            current_module_name,
            locals: DenseHashMap::new(core::ptr::null_mut()),
            work: Vec::new(),
            require_calls: Vec::new(),
        }
    }
}
