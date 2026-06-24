use crate::records::arc_collector::ArcCollector;
use luaur_ast::records::ast_type_pack::AstTypePack;

impl ArcCollector {
    pub fn visit_ast_type_pack(&mut self, node: *mut core::ffi::c_void) -> bool {
        let typed_node = node as *mut AstTypePack;
        let _ = typed_node;
        true
    }
}
