use crate::type_aliases::type_id::TypeId;
use core::ffi::c_void;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_table::AstExprTable;
use luaur_ast::records::ast_visitor::AstVisitor;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug, Clone)]
pub struct AstExprTableFinder {
    pub result: *mut DenseHashSet<TypeId>,
    pub ast_types: *const DenseHashMap<*const AstExpr, TypeId>,
}

impl AstExprTableFinder {
    pub fn new(
        result: *mut DenseHashSet<TypeId>,
        ast_types: *const DenseHashMap<*const AstExpr, TypeId>,
    ) -> Self {
        Self { result, ast_types }
    }
}

impl AstVisitor for AstExprTableFinder {
    fn visit_expr(&mut self, _node: *mut c_void) -> bool {
        false
    }

    fn visit_expr_table(&mut self, node: *mut c_void) -> bool {
        let tbl = node as *mut AstExprTable;
        unsafe {
            if let Some(ty) = (*self.ast_types).find(&(tbl as *const AstExpr)) {
                (*self.result).insert(*ty);
            }
        }
        true
    }
}
