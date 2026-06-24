use crate::records::find_node::FindNode;
use luaur_ast::records::ast_node::AstNode;

impl FindNode {
    pub fn visit_ast_node(&mut self, node: *mut AstNode) -> bool {
        let node_ref = unsafe { &*node };

        if node_ref.location.contains(self.pos) {
            self.best = node;
            return true;
        }

        if node_ref.location.end == self.document_end && self.pos >= self.document_end {
            self.best = node;
            return true;
        }

        false
    }
}
