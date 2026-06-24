use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::ast_stat_break::AstStatBreak;
use luaur_ast::records::ast_stat_if::AstStatIf;
use luaur_ast::rtti::ast_node_as;
use luaur_ast::rtti::ast_node_is;

impl TypeChecker2 {
    pub fn type_checker_2_has_break(&mut self, node: *mut AstStat) -> bool {
        unsafe {
            let block = ast_node_as::<AstStatBlock>(node as *mut AstNode);
            if !block.is_null() {
                let body = (*block).body;
                for i in 0..body.size {
                    let stat = *body.data.add(i);
                    if self.type_checker_2_has_break(stat) {
                        return true;
                    }
                }
                return false;
            }

            if ast_node_is::<AstStatBreak>(&(*node).base) {
                return true;
            }

            let if_stat = ast_node_as::<AstStatIf>(node as *mut AstNode);
            if !if_stat.is_null() {
                if self.type_checker_2_has_break((*if_stat).thenbody as *mut AstStat) {
                    return true;
                }

                if !(*if_stat).elsebody.is_null()
                    && self.type_checker_2_has_break((*if_stat).elsebody)
                {
                    return true;
                }

                return false;
            }

            false
        }
    }
}
