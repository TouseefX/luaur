use crate::records::lint_format_string::LintFormatString;
use luaur_ast::records::ast_expr_call::AstExprCall;

impl LintFormatString {
    pub fn visit_ast_expr_call(&mut self, node: *mut AstExprCall) -> bool {
        self.match_call(node);
        true
    }
}
