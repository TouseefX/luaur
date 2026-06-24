use crate::records::lint_deprecated_api::LintDeprecatedApi;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_global::AstExprGlobal;

impl LintDeprecatedApi {
    pub fn visit_ast_expr_global(&mut self, node: *mut AstExprGlobal) -> bool {
        unsafe {
            let fty = self.get_function_type(node as *mut AstExpr);
            let should_report =
                !fty.is_null() && (*fty).is_deprecated_function && !self.in_scope(fty);

            if should_report {
                if let Some(info) = (*fty).deprecated_info.as_deref() {
                    self.report_location_c_char_ast_attr_deprecated_info(
                        &(*node).base.base.location,
                        (*node).name.value,
                        info,
                    );
                } else {
                    self.report_location_c_char(&(*node).base.base.location, (*node).name.value);
                }
            }
        }

        true
    }
}
