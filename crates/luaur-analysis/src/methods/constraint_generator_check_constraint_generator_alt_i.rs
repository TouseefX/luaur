//! Source: `Analysis/src/ConstraintGenerator.cpp:3384-3412` (hand-ported)
//! C++ `Inference ConstraintGenerator::check(const ScopePtr& scope, AstExprUnary* unary)`.
use crate::enums::type_context::TypeContext;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::in_conditional_context::InConditionalContext;
use crate::records::inference::Inference;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use luaur_ast::records::ast_expr_unary::{AstExprUnary, AstExprUnaryOp};
use luaur_common::macros::luau_unreachable::LUAU_UNREACHABLE;

impl ConstraintGenerator {
    pub fn check_scope_ptr_ast_expr_unary(
        &mut self,
        scope: &ScopePtr,
        unary: *mut AstExprUnary,
    ) -> Inference {
        unsafe {
            let op = (*unary).op;

            let _in_context = if op != AstExprUnaryOp::Not {
                Some(InConditionalContext::new(
                    &mut self.type_context,
                    TypeContext::Default,
                ))
            } else {
                None
            };

            let inf = self.check_scope_ptr_ast_expr(scope, (*unary).expr);
            let operand_type = inf.ty;
            let refinement = inf.refinement;

            match op {
                AstExprUnaryOp::Not => {
                    let not_func = &(*self.builtin_types).typeFunctions.not_func as *const _;
                    let result_type = self.create_type_function_instance(
                        &*not_func,
                        alloc::vec![operand_type],
                        alloc::vec::Vec::new(),
                        scope,
                        (*unary).base.base.location,
                    );
                    let negated = self.refinement_arena.negation_refinement_id(refinement);
                    Inference::inference_type_id_refinement_id(result_type, negated)
                }
                AstExprUnaryOp::Len => {
                    let len_func = &(*self.builtin_types).typeFunctions.len_func as *const _;
                    let result_type = self.create_type_function_instance(
                        &*len_func,
                        alloc::vec![operand_type],
                        alloc::vec::Vec::new(),
                        scope,
                        (*unary).base.base.location,
                    );
                    Inference::inference_type_id_refinement_id(result_type, refinement)
                }
                AstExprUnaryOp::Minus => {
                    let unm_func = &(*self.builtin_types).typeFunctions.unm_func as *const _;
                    let result_type = self.create_type_function_instance(
                        &*unm_func,
                        alloc::vec![operand_type],
                        alloc::vec::Vec::new(),
                        scope,
                        (*unary).base.base.location,
                    );
                    Inference::inference_type_id_refinement_id(result_type, refinement)
                }
                #[allow(unreachable_patterns)]
                _ => {
                    // msvc can't prove that this is exhaustive.
                    LUAU_UNREACHABLE!()
                }
            }
        }
    }
}
