use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_stat_block::AstStatBlock;

impl AstJsonEncoder {
    pub fn visit_ast_stat_block(&mut self, node: *mut AstStatBlock) -> bool {
        self.write_ast_stat_block(node);
        false
    }
}
