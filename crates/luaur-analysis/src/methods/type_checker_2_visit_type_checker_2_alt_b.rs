use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_block::AstStatBlock;

impl TypeChecker2 {
    pub fn visit_ast_stat_block(&mut self, block: *mut AstStatBlock) {
        let _stack_pusher = self.push_stack(block as *mut AstNode);

        unsafe {
            let body = (*block).body;
            for i in 0..body.size {
                self.visit_ast_stat(*body.data.add(i));
            }
        }
    }
}
