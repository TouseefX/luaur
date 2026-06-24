use crate::enums::value_context::ValueContext;
use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_expr_instantiate::AstExprInstantiate;
use luaur_common::FFlag;

impl TypeChecker2 {
    pub fn visit_ast_expr_instantiate(
        &mut self,
        explicit_type_instantiation: *mut AstExprInstantiate,
    ) {
        unsafe {
            let expr = (*explicit_type_instantiation).expr;
            self.visit_ast_expr_value_context(expr, ValueContext::RValue);
            if FFlag::LuauExplicitTypeInstantiationSupport.get() {
                let fn_ty = self.lookup_type(expr);
                let location = (*explicit_type_instantiation).base.base.location;
                let type_arguments = (*explicit_type_instantiation).type_arguments;
                self.check_type_instantiation(expr, fn_ty, &location, type_arguments);
            }
        }
    }
}
