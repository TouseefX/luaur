use crate::records::lint_deprecated_api::LintDeprecatedApi;
use luaur_ast::records::ast_stat_function::AstStatFunction;

impl LintDeprecatedApi {
    pub fn visit_ast_stat_function(&mut self, node: *mut AstStatFunction) -> bool {
        unsafe {
            self.check_ast_expr_function((*node).func);
        }

        false
    }
}
