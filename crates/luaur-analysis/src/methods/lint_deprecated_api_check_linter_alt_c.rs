use crate::records::function_type::FunctionType;
use crate::records::lint_deprecated_api::LintDeprecatedApi;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl LintDeprecatedApi {
    pub fn check_ast_expr_function(&mut self, func: *mut AstExprFunction) {
        LUAU_ASSERT!(!func.is_null());

        let fty = self.get_function_type(func as *mut AstExpr);
        let is_deprecated = !fty.is_null() && unsafe { (*fty).is_deprecated_function };

        if is_deprecated {
            self.push_scope(fty as *const FunctionType);
        }

        unsafe {
            luaur_ast::visit::ast_expr_visit(func as *mut AstExpr, self);
        }

        if is_deprecated {
            self.pop_scope(fty as *const FunctionType);
        }
    }
}
