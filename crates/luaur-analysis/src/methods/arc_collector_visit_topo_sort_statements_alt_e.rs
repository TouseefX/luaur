use crate::functions::mk_name_topo_sort_statements_alt_i::mk_name_ast_stat_local_function;
use crate::records::arc_collector::ArcCollector;
use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;

impl ArcCollector {
    pub fn visit_ast_stat_local_function(&mut self, node: *mut core::ffi::c_void) -> bool {
        let node = unsafe { &*node.cast::<AstStatLocalFunction>() };
        let name = mk_name_ast_stat_local_function(node);
        self.add(&name);
        true
    }
}
