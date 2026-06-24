use crate::functions::dump_def::dump_def;
use crate::functions::get_local_name::get_local_name;
use crate::records::expr_printer::ExprPrinter;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_local::AstExprLocal;

impl ExprPrinter {
    pub fn visit_ast_expr_local(&mut self, node: *mut AstExprLocal) -> bool {
        unsafe {
            if let Some(def) = self.use_defs.find(&(node as *mut AstExpr)) {
                self.result.push_str(&dump_def(*def));
            } else {
                self.result.push_str(&get_local_name((*node).local));
                self.result.push('?');
            }
        }
        false
    }
}
