use crate::records::cfg_builder::CfgBuilder;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_binary::AstExprBinary;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl CfgBuilder {
    pub fn lower_expr_ast_expr(&mut self, expr: *mut AstExpr) {
        unsafe {
            let local = ast_node_as::<AstExprLocal>(expr as *mut AstNode);
            if !local.is_null() {
                self.lower_expr_ast_expr_local(local);
            } else {
                let binop = ast_node_as::<AstExprBinary>(expr as *mut AstNode);
                if !binop.is_null() {
                    LUAU_ASSERT!(!(*binop).left.is_null());
                    LUAU_ASSERT!(!(*binop).right.is_null());
                    self.lower_expr_ast_expr((*binop).left);
                    self.lower_expr_ast_expr((*binop).right);
                }
            }
        }
    }
}
