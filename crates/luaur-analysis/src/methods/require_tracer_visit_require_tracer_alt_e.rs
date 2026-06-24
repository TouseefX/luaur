use crate::records::require_tracer::RequireTracer;

impl RequireTracer {
    pub fn visit_ast_type(&mut self, _node: *mut core::ffi::c_void) -> bool {
        // allow resolving require inside `typeof` annotations
        true
    }
}
