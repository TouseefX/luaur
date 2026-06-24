use crate::functions::mk_name_topo_sort_statements_alt_e::mk_name_ast_expr_index_name;
use crate::records::arc_collector::ArcCollector;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;

impl ArcCollector {
    pub fn visit_ast_expr_index_name(&mut self, node: *mut core::ffi::c_void) -> bool {
        let node = unsafe { &*(node as *mut AstExprIndexName) };
        if let Some(name) = mk_name_ast_expr_index_name(node) {
            self.add(&name);
        }
        true
    }
}
