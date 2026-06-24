use crate::records::lint_deprecated_api::LintDeprecatedApi;
use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;

impl LintDeprecatedApi {
    pub fn visit_ast_stat_local_function(&mut self, node: *mut AstStatLocalFunction) -> bool {
        unsafe {
            self.check_ast_expr_function((*node).func);
        }

        false
    }
}
