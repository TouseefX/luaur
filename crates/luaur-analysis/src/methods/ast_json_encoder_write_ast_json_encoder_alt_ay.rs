//! Source: `Analysis/src/AstJsonEncoder.cpp:689-712` (hand-ported)
use crate::methods::ast_json_encoder_write_primitives::WriteJson;
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_block::AstStatBlock;

impl AstJsonEncoder {
    pub fn write_ast_stat_block(&mut self, node: *mut AstStatBlock) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatBlock", |e| {
            e.write_raw_string_view(",\"hasEnd\":");
            n.has_end.write_json(e);
            e.write_raw_string_view(",\"body\":[");
            let mut comma = false;
            for stat in n.body.iter() {
                if comma {
                    e.write_raw_string_view(",");
                } else {
                    comma = true;
                }
                (*stat).write_json(e);
            }
            e.write_raw_string_view("]");
        });
    }
}
