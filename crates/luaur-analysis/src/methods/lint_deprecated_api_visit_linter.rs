use crate::records::lint_deprecated_api::LintDeprecatedApi;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_local::AstExprLocal;

impl LintDeprecatedApi {
    pub fn visit_ast_expr_local(&mut self, node: *mut AstExprLocal) -> bool {
        unsafe {
            let fty = self.get_function_type(node as *mut AstExpr);
            let should_report =
                !fty.is_null() && (*fty).is_deprecated_function && !self.in_scope(fty);

            if should_report {
                if let Some(info) = (*fty).deprecated_info.as_deref() {
                    self.report_location_c_char_ast_attr_deprecated_info(
                        &(*node).base.base.location,
                        (*(*node).local).name.value,
                        info,
                    );
                } else {
                    self.report_location_c_char(
                        &(*node).base.base.location,
                        (*(*node).local).name.value,
                    );
                }
            }
        }

        true
    }
}
