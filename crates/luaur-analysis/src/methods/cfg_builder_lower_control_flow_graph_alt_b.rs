use crate::records::cfg_builder::CfgBuilder;
use luaur_ast::records::ast_stat_block::AstStatBlock;

impl CfgBuilder {
    pub fn lower_ast_stat_block(&mut self, statement: *mut AstStatBlock) {
        unsafe {
            let body = (*statement).body;
            for i in 0..body.size {
                self.lower_ast_stat(*body.data.add(i));
            }
        }
    }
}
