use crate::records::require_tracer::RequireTracer;

use core::ffi::c_void;

impl RequireTracer {
    pub fn visit_ast_expr_type_assertion(&mut self, _node: *mut c_void) -> bool {
        // suppress `require() :: any`
        false
    }
}
