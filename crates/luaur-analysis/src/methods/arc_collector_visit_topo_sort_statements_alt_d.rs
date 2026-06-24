use crate::functions::mk_name_topo_sort_statements_alt_h::mk_name_ast_stat_function;
use crate::records::arc_collector::ArcCollector;
use crate::records::internal_compiler_error::InternalCompilerError;
use luaur_ast::records::ast_stat_function::AstStatFunction;

impl ArcCollector {
    pub fn visit_ast_stat_function(&mut self, node: *mut core::ffi::c_void) -> bool {
        let node_ref = unsafe { &*(node as *mut AstStatFunction) };
        let name = mk_name_ast_stat_function(node_ref);
        self.add(&name);
        true
    }
}
