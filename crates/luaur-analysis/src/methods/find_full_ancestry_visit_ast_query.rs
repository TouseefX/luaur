use crate::records::find_full_ancestry::FindFullAncestry;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type::AstType;

impl FindFullAncestry {
    pub fn visit_ast_type(&mut self, r#type: *mut AstType) -> bool {
        if self.include_types {
            self.visit_ast_node(r#type as *mut AstNode)
        } else {
            false
        }
    }
}
