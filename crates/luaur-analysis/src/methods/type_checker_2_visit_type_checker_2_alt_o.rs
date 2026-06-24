use crate::enums::value_context::ValueContext;
use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_stat_function::AstStatFunction;
use luaur_common::FFlag;

impl TypeChecker2 {
    pub fn visit_ast_stat_function(&mut self, stat: *mut AstStatFunction) {
        unsafe {
            let name = (*stat).name;
            let func = (*stat).func;

            self.visit_ast_expr_value_context(name, ValueContext::LValue);
            self.visit_ast_expr_function(func);

            if FFlag::LuauCheckFunctionStatementTypes.get() {
                let lhs_type = self.lookup_type(name);
                let rhs_type = self.lookup_type(func as *mut AstExpr);
                let location = (*func).base.base.location;
                self.test_is_subtype_type_id_type_id_location(rhs_type, lhs_type, location);
            }
        }
    }
}
