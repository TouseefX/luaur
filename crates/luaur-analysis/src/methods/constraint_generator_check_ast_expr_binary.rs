use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::equality_constraint::EqualityConstraint;
use crate::records::inference::Inference;
use crate::records::type_function::TypeFunction;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_binary::AstExprBinaryOp;
use luaur_ast::records::location::Location;
use luaur_common::macros::luau_unreachable::LUAU_UNREACHABLE;

impl ConstraintGenerator {
    pub fn check_ast_expr_binary(
        &mut self,
        scope: &ScopePtr,
        location: Location,
        op: AstExprBinaryOp,
        left: *mut AstExpr,
        right: *mut AstExpr,
        expected_type: Option<TypeId>,
    ) -> Inference {
        let (left_type, right_type, refinement) =
            self.check_binary(scope, op, left, right, expected_type);

        match op {
            AstExprBinaryOp::Add => {
                let result_type = self.create_type_function_instance(
                    unsafe { &(*self.builtin_types).typeFunctions.add_func },
                    Vec::from([left_type, right_type]),
                    Vec::new(),
                    scope,
                    location,
                );
                Inference::inference_type_id_refinement_id(result_type, refinement)
            }
            AstExprBinaryOp::Sub => {
                let result_type = self.create_type_function_instance(
                    unsafe { &(*self.builtin_types).typeFunctions.sub_func },
                    Vec::from([left_type, right_type]),
                    Vec::new(),
                    scope,
                    location,
                );
                Inference::inference_type_id_refinement_id(result_type, refinement)
            }
            AstExprBinaryOp::Mul => {
                let result_type = self.create_type_function_instance(
                    unsafe { &(*self.builtin_types).typeFunctions.mul_func },
                    Vec::from([left_type, right_type]),
                    Vec::new(),
                    scope,
                    location,
                );
                Inference::inference_type_id_refinement_id(result_type, refinement)
            }
            AstExprBinaryOp::Div => {
                let result_type = self.create_type_function_instance(
                    unsafe { &(*self.builtin_types).typeFunctions.div_func },
                    Vec::from([left_type, right_type]),
                    Vec::new(),
                    scope,
                    location,
                );
                Inference::inference_type_id_refinement_id(result_type, refinement)
            }
            AstExprBinaryOp::FloorDiv => {
                let result_type = self.create_type_function_instance(
                    unsafe { &(*self.builtin_types).typeFunctions.idiv_func },
                    Vec::from([left_type, right_type]),
                    Vec::new(),
                    scope,
                    location,
                );
                Inference::inference_type_id_refinement_id(result_type, refinement)
            }
            AstExprBinaryOp::Pow => {
                let result_type = self.create_type_function_instance(
                    unsafe { &(*self.builtin_types).typeFunctions.pow_func },
                    Vec::from([left_type, right_type]),
                    Vec::new(),
                    scope,
                    location,
                );
                Inference::inference_type_id_refinement_id(result_type, refinement)
            }
            AstExprBinaryOp::Mod => {
                let result_type = self.create_type_function_instance(
                    unsafe { &(*self.builtin_types).typeFunctions.mod_func },
                    Vec::from([left_type, right_type]),
                    Vec::new(),
                    scope,
                    location,
                );
                Inference::inference_type_id_refinement_id(result_type, refinement)
            }
            AstExprBinaryOp::Concat => {
                let result_type = self.create_type_function_instance(
                    unsafe { &(*self.builtin_types).typeFunctions.concat_func },
                    Vec::from([left_type, right_type]),
                    Vec::new(),
                    scope,
                    location,
                );
                Inference::inference_type_id_refinement_id(result_type, refinement)
            }
            AstExprBinaryOp::And => {
                let result_type = self.create_type_function_instance(
                    unsafe { &(*self.builtin_types).typeFunctions.and_func },
                    Vec::from([left_type, right_type]),
                    Vec::new(),
                    scope,
                    location,
                );
                Inference::inference_type_id_refinement_id(result_type, refinement)
            }
            AstExprBinaryOp::Or => {
                let result_type = self.create_type_function_instance(
                    unsafe { &(*self.builtin_types).typeFunctions.or_func },
                    Vec::from([left_type, right_type]),
                    Vec::new(),
                    scope,
                    location,
                );
                Inference::inference_type_id_refinement_id(result_type, refinement)
            }
            AstExprBinaryOp::CompareLt
            | AstExprBinaryOp::CompareGe
            | AstExprBinaryOp::CompareLe
            | AstExprBinaryOp::CompareGt => {
                self.add_constraint_scope_ptr_location_constraint_v(
                    scope,
                    location,
                    ConstraintV::Equality(EqualityConstraint {
                        result_type: left_type,
                        assignment_type: right_type,
                    }),
                );
                Inference::inference_type_id_refinement_id(
                    unsafe { (*self.builtin_types).booleanType },
                    refinement,
                )
            }
            AstExprBinaryOp::CompareEq | AstExprBinaryOp::CompareNe => {
                Inference::inference_type_id_refinement_id(
                    unsafe { (*self.builtin_types).booleanType },
                    refinement,
                )
            }
            AstExprBinaryOp::Op__Count => {
                unsafe { (*self.ice).ice_string("Op__Count should never be generated in an AST.") };
                LUAU_UNREACHABLE!()
            }
        }
    }
}
