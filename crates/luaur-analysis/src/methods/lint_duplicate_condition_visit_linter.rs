use crate::records::lint_duplicate_condition::LintDuplicateCondition;
use alloc::vec::Vec;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_if::AstStatIf;
use luaur_ast::rtti::ast_node_as;
use luaur_ast::visit::{ast_expr_visit, ast_stat_visit};

impl LintDuplicateCondition {
    pub fn visit_ast_stat_if(&mut self, stat: *mut AstStatIf) -> bool {
        unsafe {
            if stat.is_null() || (*stat).elsebody.is_null() {
                return true;
            }

            if ast_node_as::<AstStatIf>((*stat).elsebody as *mut AstNode).is_null() {
                return true;
            }

            let mut conditions = Vec::with_capacity(2);
            let mut head = stat;

            while !head.is_null() {
                ast_expr_visit((*head).condition, self);
                ast_stat_visit((*head).thenbody as *mut _, self);

                conditions.push((*head).condition);

                if !(*head).elsebody.is_null() {
                    let next = ast_node_as::<AstStatIf>((*head).elsebody as *mut AstNode);
                    if !next.is_null() {
                        head = next;
                        continue;
                    }

                    ast_stat_visit((*head).elsebody, self);
                }

                break;
            }

            self.detect_duplicates(&conditions);
        }

        false
    }
}
