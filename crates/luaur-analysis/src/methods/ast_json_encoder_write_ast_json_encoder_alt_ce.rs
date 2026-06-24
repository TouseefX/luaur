//! Source: `Analysis/src/AstJsonEncoder.cpp:1126-1136` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type_pack_explicit::AstTypePackExplicit;

impl AstJsonEncoder {
    pub fn write_ast_type_pack_explicit(&mut self, node: *mut AstTypePackExplicit) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstTypePackExplicit", |e| {
            e.write("typeList", &n.type_list);
        });
    }
}
