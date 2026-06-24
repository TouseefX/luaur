//! Source: `Analysis/src/AstJsonEncoder.cpp:865-876` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_function::AstStatFunction;

impl AstJsonEncoder {
    pub fn write_ast_stat_function(&mut self, node: *mut AstStatFunction) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatFunction", |e| {
            e.write("name", &n.name);
            e.write("func", &n.func);
        });
    }
}
