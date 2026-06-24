//! Source: `Analysis/src/AstJsonEncoder.cpp:928-940` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_declare_global::AstStatDeclareGlobal;

impl AstJsonEncoder {
    pub fn write_ast_stat_declare_global(&mut self, node: *mut AstStatDeclareGlobal) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatDeclareGlobal", |e| {
            e.write("name", &n.name);
            e.write("nameLocation", &n.name_location);
            e.write("type", &n.type_);
        });
    }
}
