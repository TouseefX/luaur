//! Source: `Analysis/src/AstJsonEncoder.cpp:1138-1148` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type_pack_variadic::AstTypePackVariadic;

impl AstJsonEncoder {
    pub fn write_ast_type_pack_variadic(&mut self, node: *mut AstTypePackVariadic) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstTypePackVariadic", |e| {
            e.write("variadicType", &n.variadic_type);
        });
    }
}
