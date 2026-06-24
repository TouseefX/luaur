//! Source: `Analysis/src/AstJsonEncoder.cpp:851-863` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_compound_assign::AstStatCompoundAssign;

impl AstJsonEncoder {
    pub fn write_ast_stat_compound_assign(&mut self, node: *mut AstStatCompoundAssign) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(
            node as *mut AstNode,
            "AstStatCompoundAssign",
            |e| {
                e.write("op", &n.op);
                e.write("var", &n.var);
                e.write("value", &n.value);
            },
        );
    }
}
