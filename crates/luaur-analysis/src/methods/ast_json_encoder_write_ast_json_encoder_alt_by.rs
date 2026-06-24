//! Source: `Analysis/src/AstJsonEncoder.cpp:1055-1070` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type_function::AstTypeFunction;

impl AstJsonEncoder {
    pub fn write_ast_type_function(&mut self, node: *mut AstTypeFunction) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstTypeFunction", |e| {
            e.write("attributes", &n.attributes);
            e.write("generics", &n.generics);
            e.write("genericPacks", &n.generic_packs);
            e.write("argTypes", &n.arg_types);
            e.write("argNames", &n.arg_names);
            e.write("returnTypes", &n.return_types);
        });
    }
}
