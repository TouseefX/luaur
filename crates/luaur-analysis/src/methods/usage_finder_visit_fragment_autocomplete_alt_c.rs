use crate::records::usage_finder::UsageFinder;
use luaur_ast::records::ast_type_pack::AstTypePack;

impl UsageFinder {
    pub fn visit_ast_type_pack(&mut self, _node: *mut AstTypePack) -> bool {
        true
    }
}
