//! Source: `Analysis/src/AstJsonEncoder.cpp:394-407` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_node::AstNode;

impl AstJsonEncoder {
    pub fn write_ast_expr_call(&mut self, node: *mut AstExprCall) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstExprCall", |e| {
            e.write("func", &n.func);
            e.write("args", &n.args);
            e.write("self", &n.self_);
            e.write("argLocation", &n.arg_location);
        });
    }
}
