use crate::records::global_name_collector::GlobalNameCollector;
use luaur_ast::records::ast_expr_global::AstExprGlobal;

impl GlobalNameCollector {
    pub fn visit(&mut self, node: *mut AstExprGlobal) -> bool {
        let node_ref = unsafe { &*node };
        self.names.insert(node_ref.name.clone());
        true
    }
}
