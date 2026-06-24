use crate::functions::mk_name_topo_sort_statements_alt_d::mk_name_ast_name;
use crate::records::arc_collector::ArcCollector;
use luaur_ast::records::ast_type_reference::AstTypeReference;

impl ArcCollector {
    pub fn visit_ast_type_reference(&mut self, node: *mut core::ffi::c_void) -> bool {
        let node = unsafe { node as *mut AstTypeReference };
        let name = unsafe { (*node).name };
        let identifier = mk_name_ast_name(&name);
        self.add(&identifier);
        true
    }
}
