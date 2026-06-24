use crate::records::usage_finder::UsageFinder;
use luaur_ast::records::ast_type::AstType;

impl UsageFinder {
    pub fn visit_ast_type(&mut self, _node: *mut AstType) -> bool {
        true
    }
}
