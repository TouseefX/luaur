use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::inference::Inference;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use luaur_ast::records::ast_expr_type_assertion::AstExprTypeAssertion;

impl ConstraintGenerator {
    pub fn check_scope_ptr_ast_expr_type_assertion(
        &mut self,
        scope: &ScopePtr,
        type_assert: *mut AstExprTypeAssertion,
    ) -> Inference {
        let type_assert_ref = unsafe { &*type_assert };
        self.check_scope_ptr_ast_expr(scope, type_assert_ref.expr);
        Inference::inference_type_id_refinement_id(
            self.resolve_type(
                unsafe { scope.as_ref() as *const _ as *mut _ },
                type_assert_ref.annotation,
                false,
                false,
                crate::enums::polarity::Polarity::Positive,
            ),
            core::ptr::null_mut(),
        )
    }
}
