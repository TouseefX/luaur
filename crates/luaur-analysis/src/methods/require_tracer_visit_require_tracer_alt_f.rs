use crate::records::require_tracer::RequireTracer;
use luaur_ast::records::ast_type_pack::AstTypePack;

impl RequireTracer {
    pub fn visit_ast_type_pack(&mut self, _node: *mut core::ffi::c_void) -> bool {
        true
    }
}
