use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::ast_visitor::AstVisitor;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug, Clone)]
pub struct GlobalNameCollector {
    pub(crate) names: DenseHashSet<AstName>,
}

impl AstVisitor for GlobalNameCollector {
    fn visit_expr_global(&mut self, node: *mut core::ffi::c_void) -> bool {
        let node = node as *mut AstExprGlobal;
        unsafe {
            self.names.insert((*node).name);
        }
        true
    }
}

impl GlobalNameCollector {
    pub fn new() -> Self {
        Self {
            names: DenseHashSet::new(AstName::new()),
        }
    }
}

impl Default for GlobalNameCollector {
    fn default() -> Self {
        Self::new()
    }
}
