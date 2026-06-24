use crate::records::ast_expr_table_finder::AstExprTableFinder;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl AstExprTableFinder {
    pub fn ast_expr_table_finder(
        &mut self,
        result: *mut DenseHashSet<TypeId>,
        ast_types: *const DenseHashMap<*const AstExpr, TypeId>,
    ) {
        self.result = result;
        self.ast_types = ast_types;
    }
}
