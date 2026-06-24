use crate::records::global_prepopulator::GlobalPrepopulator;
use crate::records::symbol::Symbol;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_global::AstExprGlobal;

impl GlobalPrepopulator {
    pub fn visit_ast_expr_global(&mut self, global: *mut AstExprGlobal) -> bool {
        unsafe {
            let global_name = (*global).name;
            let scope = self.global_scope.as_mut();

            if let Some(ty) = scope.lookup_symbol(Symbol::from_global(global_name)) {
                let def = self.dfg.as_ref().get_def(global as *const AstExpr);
                *scope.lvalue_types.get_or_insert(def) = ty;
            }
        }

        true
    }
}
