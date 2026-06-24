use crate::records::find_expr_or_local::FindExprOrLocal;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_function::AstExprFunction;

impl FindExprOrLocal {
    pub fn visit_ast_expr_function(&mut self, fn_: *mut AstExprFunction) -> bool {
        unsafe {
            for i in 0..(*fn_).args.size {
                self.visit_ast_local(*((*fn_).args.data.add(i)));
            }
        }
        self.visit_ast_expr(fn_ as *mut AstExpr)
    }
}
