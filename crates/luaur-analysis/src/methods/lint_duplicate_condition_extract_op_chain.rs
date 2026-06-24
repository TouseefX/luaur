use crate::records::lint_duplicate_condition::LintDuplicateCondition;
use alloc::vec::Vec;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_binary::{AstExprBinary, AstExprBinary_Op};
use luaur_ast::records::ast_expr_group::AstExprGroup;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;

impl LintDuplicateCondition {
    pub fn extract_op_chain(
        &mut self,
        conditions: &mut Vec<*mut AstExpr>,
        expr: *mut AstExpr,
        op: AstExprBinary_Op,
    ) {
        unsafe {
            let bin = ast_node_as::<AstExprBinary>(expr as *mut AstNode);
            if !bin.is_null() && (*bin).op == op {
                self.extract_op_chain(conditions, (*bin).left, op);
                self.extract_op_chain(conditions, (*bin).right, op);
                return;
            }

            let group = ast_node_as::<AstExprGroup>(expr as *mut AstNode);
            if !group.is_null() {
                self.extract_op_chain(conditions, (*group).expr, op);
                return;
            }
        }

        conditions.push(expr);
    }
}
