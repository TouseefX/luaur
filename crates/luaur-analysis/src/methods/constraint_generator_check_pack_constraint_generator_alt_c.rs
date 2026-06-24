use crate::enums::type_context::TypeContext;
use crate::functions::checkpoint::checkpoint;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::in_conditional_context::InConditionalContext;
use crate::records::inference_pack::InferencePack;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use luaur_ast::records::ast_expr_call::AstExprCall;

impl ConstraintGenerator {
    pub fn check_pack_scope_ptr_ast_expr_call(
        &mut self,
        scope: &ScopePtr,
        call: *mut AstExprCall,
    ) -> InferencePack {
        let func_begin = checkpoint(self);
        let fn_type = {
            let _in_context =
                InConditionalContext::new(&mut self.type_context, TypeContext::Default);
            self.check_scope_ptr_ast_expr(scope, unsafe { (*call).func })
                .ty
        };
        let func_end = checkpoint(self);
        self.check_expr_call(scope, call, fn_type, func_begin, func_end)
    }
}
