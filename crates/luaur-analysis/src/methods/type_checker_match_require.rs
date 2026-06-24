use crate::functions::match_require::match_require;
use crate::records::type_checker::TypeChecker;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_call::AstExprCall;

impl TypeChecker {
    pub fn match_require(&mut self, call: &AstExprCall) -> Option<*mut AstExpr> {
        match_require(call)
    }
}
