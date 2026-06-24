use crate::records::lint_duplicate_condition::LintDuplicateCondition;
use alloc::vec::Vec;
use luaur_ast::records::ast_expr_if_else::AstExprIfElse;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;
use luaur_ast::visit::ast_expr_visit;

impl LintDuplicateCondition {
    pub fn visit_ast_expr_if_else(&mut self, expr: *mut AstExprIfElse) -> bool {
        unsafe {
            if expr.is_null()
                || ast_node_as::<AstExprIfElse>((*expr).false_expr as *mut AstNode).is_null()
            {
                return true;
            }

            let mut conditions = Vec::with_capacity(2);
            let mut head = expr;

            while !head.is_null() {
                ast_expr_visit((*head).condition, self);
                ast_expr_visit((*head).true_expr, self);

                conditions.push((*head).condition);

                let next = ast_node_as::<AstExprIfElse>((*head).false_expr as *mut AstNode);
                if !next.is_null() {
                    head = next;
                    continue;
                }

                ast_expr_visit((*head).false_expr, self);
                break;
            }

            self.detect_duplicates(&conditions);
        }

        false
    }
}
