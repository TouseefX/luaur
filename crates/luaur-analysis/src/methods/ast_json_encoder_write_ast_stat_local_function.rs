//! Source: `Analysis/src/AstJsonEncoder.cpp:878-889` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;

impl AstJsonEncoder {
    pub fn write_ast_stat_local_function(&mut self, node: *mut AstStatLocalFunction) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatLocalFunction", |e| {
            e.write("name", &n.name);
            e.write("func", &n.func);
        });
    }
}
