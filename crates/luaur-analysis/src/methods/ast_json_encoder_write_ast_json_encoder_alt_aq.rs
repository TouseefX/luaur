//! Source: `Analysis/src/AstJsonEncoder.cpp:557-568` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_interp_string::AstExprInterpString;
use luaur_ast::records::ast_node::AstNode;

impl AstJsonEncoder {
    pub fn write_ast_expr_interp_string(&mut self, node: *mut AstExprInterpString) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstExprInterpString", |e| {
            e.write("strings", &n.strings);
            e.write("expressions", &n.expressions);
        });
    }
}
