use crate::records::find_expr_or_local::FindExprOrLocal;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::visit;

impl FindExprOrLocal {
    pub fn visit_ast_stat_block(&mut self, block: *mut AstStatBlock) -> bool {
        unsafe {
            for stat in (*block).body.iter() {
                let stat = *stat;
                let stat_node = stat as *mut AstNode;
                if (*stat_node).location.end <= self.pos {
                    continue;
                }
                if (*stat_node).location.begin > self.pos {
                    break;
                }

                visit::ast_stat_visit(stat, self);
            }
        }

        false
    }
}
