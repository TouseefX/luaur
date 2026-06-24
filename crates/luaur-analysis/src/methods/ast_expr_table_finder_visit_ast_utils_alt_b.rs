use crate::records::ast_expr_table_finder::AstExprTableFinder;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr_table::AstExprTable;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl AstExprTableFinder {
    pub fn visit_ast_expr_table(&mut self, tbl: *mut AstExprTable) -> bool {
        unsafe {
            let ty = (*self.ast_types).find(&(tbl as *const luaur_ast::records::ast_expr::AstExpr));
            LUAU_ASSERT!(ty.is_some());
            if let Some(ty) = ty {
                (*self.result).insert(*ty);
            }
        }
        true
    }
}
