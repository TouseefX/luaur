use crate::functions::mk_name_topo_sort_statements_alt_c::mk_name_ast_expr_global;
use crate::records::arc_collector::ArcCollector;
use crate::records::identifier::Identifier;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_name::AstName;

impl ArcCollector {
    pub fn visit_ast_expr_global(&mut self, node: *mut core::ffi::c_void) -> bool {
        let node = node as *mut AstExprGlobal;
        let name: Identifier = unsafe { mk_name_ast_expr_global(&*node) };
        self.add(&name);
        true
    }
}
