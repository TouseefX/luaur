use crate::records::lint_table_literal::LintTableLiteral;
use luaur_ast::records::ast_type_pack::AstTypePack;

impl LintTableLiteral {
    pub fn visit_ast_type_pack(&mut self, _node: *mut AstTypePack) -> bool {
        true
    }
}
