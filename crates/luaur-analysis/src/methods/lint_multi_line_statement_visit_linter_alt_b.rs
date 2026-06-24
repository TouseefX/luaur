use luaur_ast::records::ast_expr_table::AstExprTable;

impl crate::records::lint_multi_line_statement::LintMultiLineStatement {
    pub fn visit_ast_expr_table(&mut self, _node: *mut AstExprTable) -> bool {
        false
    }
}
