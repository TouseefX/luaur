use crate::functions::mk_name_topo_sort_statements_alt_g::mk_name_ast_expr;
use crate::records::arc_collector::ArcCollector;
use luaur_ast::records::ast_type_typeof::AstTypeTypeof;

impl ArcCollector {
    pub fn visit_ast_type_typeof(&mut self, node: *mut core::ffi::c_void) -> bool {
        let node = unsafe { node as *mut AstTypeTypeof };
        let expr = unsafe { (*node).expr };
        let name = mk_name_ast_expr(unsafe { &*expr });
        if let Some(name) = name {
            self.add(&name);
        }
        true
    }
}
