use crate::records::lint_duplicate_condition::LintDuplicateCondition;
use alloc::vec::Vec;
use luaur_ast::records::ast_expr_binary::{AstExprBinary, AstExprBinary_Op};
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;
use luaur_ast::visit::ast_expr_visit;

impl LintDuplicateCondition {
    pub fn visit_ast_expr_binary(&mut self, expr: *mut AstExprBinary) -> bool {
        unsafe {
            if expr.is_null()
                || ((*expr).op != AstExprBinary_Op::And && (*expr).op != AstExprBinary_Op::Or)
            {
                return true;
            }

            if (*expr).op == AstExprBinary_Op::Or {
                let la = ast_node_as::<AstExprBinary>((*expr).left as *mut AstNode);

                if !la.is_null() && (*la).op == AstExprBinary_Op::And {
                    let lb = ast_node_as::<AstExprBinary>((*la).left as *mut AstNode);
                    let rb = ast_node_as::<AstExprBinary>((*la).right as *mut AstNode);

                    if !(lb.is_null() || (*lb).op != AstExprBinary_Op::And)
                        || !(rb.is_null() || (*rb).op != AstExprBinary_Op::And)
                    {
                        // This is an and-chain longer than two; continue with duplicate detection.
                    } else {
                        ast_expr_visit((*la).left, self);
                        ast_expr_visit((*la).right, self);
                        ast_expr_visit((*expr).right, self);
                        return false;
                    }
                }
            }

            let mut conditions = Vec::with_capacity(2);
            self.extract_op_chain(&mut conditions, expr as *mut _, (*expr).op);
            self.detect_duplicates(&conditions);
        }

        false
    }
}
