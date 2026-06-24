use crate::records::arc_collector::ArcCollector;

impl ArcCollector {
    pub fn visit_ast_type(&mut self, _node: *mut core::ffi::c_void) -> bool {
        true
    }
}
