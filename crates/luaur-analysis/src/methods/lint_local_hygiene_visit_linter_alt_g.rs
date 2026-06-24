use crate::records::lint_local_hygiene::LintLocalHygiene;
use luaur_ast::records::ast_type_pack::AstTypePack;

impl LintLocalHygiene {
    pub fn visit_ast_type_pack(&mut self, node: *mut AstTypePack) -> bool {
        let _ = node;
        true
    }
}
