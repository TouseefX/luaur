use crate::records::lint_table_operations::LintTableOperations;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_constant_number::AstExprConstantNumber;
use luaur_ast::rtti::ast_node_as;

impl LintTableOperations {
    pub fn is_constant(&mut self, expr: *mut AstExpr, value: f64) -> bool {
        let n = unsafe { ast_node_as::<AstExprConstantNumber>(expr as *mut _) };
        if !n.is_null() {
            unsafe { (*n).value == value }
        } else {
            false
        }
    }
}
