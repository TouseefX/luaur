use crate::enums::control_flow::ControlFlow;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use luaur_ast::records::ast_stat_error::AstStatError;

impl ConstraintGenerator {
    pub fn visit_scope_ptr_ast_stat_error(
        &mut self,
        scope: &ScopePtr,
        error: *mut AstStatError,
    ) -> ControlFlow {
        unsafe {
            for i in 0..(*error).statements.size {
                self.visit_scope_ptr_ast_stat(scope, *(*error).statements.data.add(i));
            }
            for i in 0..(*error).expressions.size {
                let expr = *(*error).expressions.data.add(i);
                self.check_scope_ptr_ast_expr(scope, expr);
            }
        }
        ControlFlow::None
    }
}
