use crate::records::file_resolver::FileResolver;
use crate::records::require_trace_result::RequireTraceResult;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::type_aliases::module_name_file_resolver::ModuleName;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::visit::AstVisitable;

use crate::records::require_tracer::RequireTracer;

pub fn trace_requires(
    file_resolver: *mut FileResolver,
    root: *mut AstStatBlock,
    current_module_name: ModuleName,
    limits: &TypeCheckLimits,
) -> RequireTraceResult {
    let mut result = RequireTraceResult {
        exprs: luaur_common::records::dense_hash_map::DenseHashMap::new(core::ptr::null_mut()),
        require_list: alloc::vec::Vec::new(),
    };

    let mut tracer = RequireTracer::require_tracer(
        &mut result as *mut RequireTraceResult,
        file_resolver,
        current_module_name,
    );

    unsafe {
        if let Some(root_ref) = root.as_mut() {
            root_ref.visit(&mut tracer);
        }
    }

    tracer.process(limits);

    result
}
