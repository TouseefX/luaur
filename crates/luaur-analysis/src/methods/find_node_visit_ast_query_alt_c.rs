use crate::records::find_node::FindNode;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::visit::ast_stat_visit;

impl FindNode {
    pub fn visit_ast_stat_block(&mut self, block: *mut AstStatBlock) -> bool {
        self.visit_ast_node(block as *mut AstNode);

        let block_ref = unsafe { &*block };
        let body = block_ref.body;

        for i in 0..body.size {
            let stat = unsafe { *body.data.add(i) };
            let stat_ref = unsafe { &*stat };

            if stat_ref.base.location.end < self.pos {
                continue;
            }
            if stat_ref.base.location.begin > self.pos {
                break;
            }

            unsafe {
                ast_stat_visit(stat, self);
            }
        }

        false
    }
}
