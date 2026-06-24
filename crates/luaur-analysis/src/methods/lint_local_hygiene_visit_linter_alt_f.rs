use crate::records::lint_local_hygiene::LintLocalHygiene;
use luaur_ast::records::ast_type::AstType;

impl LintLocalHygiene {
    pub fn visit_ast_type(&mut self, node: *mut AstType) -> bool {
        let _ = node;
        true
    }
}
