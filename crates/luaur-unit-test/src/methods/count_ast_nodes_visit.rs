use crate::records::count_ast_nodes::CountAstNodes;
use luaur_ast::records::ast_node::AstNode;

impl CountAstNodes {
    pub fn visit(&mut self, _node: *mut AstNode) -> bool {
        self.count += 1;
        true
    }
}
