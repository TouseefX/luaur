//! Source: `Analysis/src/AstJsonEncoder.cpp:1024-1035` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type_table::AstTypeTable;

impl AstJsonEncoder {
    pub fn write_ast_type_table(&mut self, node: *mut AstTypeTable) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstTypeTable", |e| {
            e.write("props", &n.props);
            e.write("indexer", &n.indexer);
        });
    }
}
