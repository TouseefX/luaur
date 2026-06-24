use crate::records::lint_multi_line_statement::LintMultiLineStatement;

use luaur_ast::records::ast_stat_block::AstStatBlock;

impl LintMultiLineStatement {
    pub fn visit_ast_stat_block(&mut self, node: *mut AstStatBlock) -> bool {
        let node_body = unsafe { (*node).body };
        for i in 0..node_body.size {
            let stmt = unsafe { *node_body.data.add(i) };

            let s = crate::records::statement::Statement {
                start: unsafe { (*stmt).base.location },
                lastLine: unsafe { (*stmt).base.location.begin.line },
                flagged: false,
            };

            self.stack.push(s);

            unsafe {
                luaur_ast::visit::ast_stat_visit(stmt, self);
            }

            self.stack.pop();
        }

        false
    }
}
