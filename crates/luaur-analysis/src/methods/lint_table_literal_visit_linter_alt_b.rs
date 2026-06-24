use crate::records::lint_table_literal::LintTableLiteral;
use luaur_ast::records::ast_type::AstType;

impl LintTableLiteral {
    pub fn visit_ast_type(&mut self, node: *mut AstType) -> bool {
        let _ = node;
        true
    }
}
