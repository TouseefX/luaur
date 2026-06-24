//! Source: `Analysis/src/ConstraintGenerator.cpp:3540-3548` (hand-ported)
//! C++ `Inference ConstraintGenerator::check(const ScopePtr& scope, AstExprInterpString* interpString)`.
use crate::enums::type_context::TypeContext;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::in_conditional_context::InConditionalContext;
use crate::records::inference::Inference;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use luaur_ast::records::ast_expr_interp_string::AstExprInterpString;

impl ConstraintGenerator {
    pub fn check_scope_ptr_ast_expr_interp_string(
        &mut self,
        scope: &ScopePtr,
        interp_string: *mut AstExprInterpString,
    ) -> Inference {
        unsafe {
            let _in_context =
                InConditionalContext::new(&mut self.type_context, TypeContext::Default);

            let expressions = (*interp_string).expressions;
            for i in 0..expressions.size as usize {
                let expr = *expressions.data.add(i);
                self.check_scope_ptr_ast_expr(scope, expr);
            }

            Inference::inference_type_id_refinement_id(
                (*self.builtin_types).stringType,
                core::ptr::null_mut(),
            )
        }
    }
}
