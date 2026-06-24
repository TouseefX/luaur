use crate::records::lint_unreachable_code::LintUnreachableCode;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_stat::AstStat;

impl LintUnreachableCode {
    pub fn visit(&mut self, node: *mut AstExprFunction) -> bool {
        let node_ref = unsafe { &*node };
        if !node_ref.body.is_null() {
            self.analyze(node_ref.body as *mut AstStat);
        }
        true
    }
}
