use crate::records::lint_context::LintContext;
use luaur_ast::records::ast_expr_binary::AstExprBinary;
use luaur_ast::records::ast_visitor::AstVisitor;

#[derive(Debug, Clone)]
pub struct LintUnknownType {
    pub(crate) context: *mut LintContext,
}

impl AstVisitor for LintUnknownType {
    fn visit_expr_binary(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit(node as *mut AstExprBinary)
    }
}
