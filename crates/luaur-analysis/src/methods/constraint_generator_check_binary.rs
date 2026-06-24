//! Source: `Analysis/src/ConstraintGenerator.cpp:3606-3731` (hand-ported)
//! C++ `std::tuple<TypeId, TypeId, RefinementId> ConstraintGenerator::checkBinary(...)`.
use crate::enums::type_context::TypeContext;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::has_tag_type_alt_b::has_tag;
use crate::functions::match_type_guard::match_type_guard;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::extern_type::ExternType;
use crate::records::in_conditional_context::InConditionalContext;
use crate::records::union_type::UnionType;
use crate::type_aliases::refinement_id_refinement::RefinementId;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_binary::{AstExprBinary, AstExprBinaryOp};
use luaur_ast::records::ast_node::AstNode;

impl ConstraintGenerator {
    pub fn check_binary(
        &mut self,
        scope: &ScopePtr,
        op: AstExprBinaryOp,
        left: *mut AstExpr,
        right: *mut AstExpr,
        expected_type: Option<TypeId>,
    ) -> (TypeId, TypeId, RefinementId) {
        unsafe {
            let _in_context = if op != AstExprBinary::And
                && op != AstExprBinary::Or
                && op != AstExprBinary::CompareEq
                && op != AstExprBinary::CompareNe
            {
                Some(InConditionalContext::new(
                    &mut self.type_context,
                    TypeContext::Default,
                ))
            } else {
                None
            };

            if op == AstExprBinary::And {
                let mut relaxed_expected_lhs: Option<TypeId> = None;

                if let Some(exp) = expected_type {
                    relaxed_expected_lhs = Some((*self.arena).add_type(UnionType {
                        options: alloc::vec![(*self.builtin_types).falsyType, exp],
                    }));
                }

                let left_inf = self.check_scope_ptr_ast_expr_optional_type_id(
                    scope,
                    left,
                    relaxed_expected_lhs,
                );
                let left_type = left_inf.ty;
                let left_refinement = left_inf.refinement;

                let right_scope = self.child_scope(right as *mut AstNode, scope);
                self.apply_refinements(&right_scope, (*right).base.location, left_refinement);
                let right_inf = self.check_scope_ptr_ast_expr_optional_type_id(
                    &right_scope,
                    right,
                    expected_type,
                );
                let right_type = right_inf.ty;
                let right_refinement = right_inf.refinement;

                let conj = self
                    .refinement_arena
                    .conjunction_refinement_id_refinement_id(left_refinement, right_refinement);
                return (left_type, right_type, conj);
            } else if op == AstExprBinary::Or {
                let mut relaxed_expected_lhs: Option<TypeId> = None;

                if let Some(exp) = expected_type {
                    relaxed_expected_lhs = Some((*self.arena).add_type(UnionType {
                        options: alloc::vec![(*self.builtin_types).falsyType, exp],
                    }));
                }

                let left_inf = self.check_scope_ptr_ast_expr_optional_type_id(
                    scope,
                    left,
                    relaxed_expected_lhs,
                );
                let left_type = left_inf.ty;
                let left_refinement = left_inf.refinement;

                let right_scope = self.child_scope(right as *mut AstNode, scope);
                let negated = self
                    .refinement_arena
                    .negation_refinement_id(left_refinement);
                self.apply_refinements(&right_scope, (*right).base.location, negated);
                let right_inf = self.check_scope_ptr_ast_expr_optional_type_id(
                    &right_scope,
                    right,
                    expected_type,
                );
                let right_type = right_inf.ty;
                let right_refinement = right_inf.refinement;

                let disj = self
                    .refinement_arena
                    .disjunction_refinement_id_refinement_id(left_refinement, right_refinement);
                return (left_type, right_type, disj);
            } else if let Some(typeguard) = match_type_guard(op as i32, left, right) {
                let left_type = self.check_scope_ptr_ast_expr(scope, left).ty;
                let right_type = self.check_scope_ptr_ast_expr(scope, right).ty;

                let key = (*self.dfg).get_refinement_key(typeguard.target as *const AstExpr);
                if key.is_null() {
                    return (left_type, right_type, core::ptr::null_mut());
                }

                let mut discriminant_ty: TypeId = (*self.builtin_types).neverType;
                let guard_type = typeguard.r#type();
                if guard_type == "nil" {
                    discriminant_ty = (*self.builtin_types).nilType;
                } else if guard_type == "string" {
                    discriminant_ty = (*self.builtin_types).stringType;
                } else if guard_type == "number" {
                    discriminant_ty = (*self.builtin_types).numberType;
                } else if guard_type == "integer" {
                    discriminant_ty = (*self.builtin_types).integerType;
                } else if guard_type == "boolean" {
                    discriminant_ty = (*self.builtin_types).booleanType;
                } else if guard_type == "thread" {
                    discriminant_ty = (*self.builtin_types).threadType;
                } else if guard_type == "buffer" {
                    discriminant_ty = (*self.builtin_types).bufferType;
                } else if guard_type == "table" {
                    discriminant_ty = (*self.builtin_types).tableType;
                } else if guard_type == "function" {
                    discriminant_ty = (*self.builtin_types).functionType;
                } else if guard_type == "userdata" {
                    // For now, we don't really care about being accurate with userdata if the typeguard was using typeof.
                    discriminant_ty = (*self.builtin_types).externType;
                } else if guard_type == "vector" && !typeguard.isTypeof() {
                    // `vector` is defined in EmbeddedBuiltinDefinitions, not as an actual built-in type
                    let type_fun = self
                        .global_scope
                        .as_ref()
                        .unwrap()
                        .lookup_type(&alloc::string::String::from("vector"));
                    if let Some(type_fun) = type_fun {
                        discriminant_ty = follow_type_id(type_fun.r#type());
                    }
                } else if !typeguard.isTypeof() {
                    discriminant_ty = (*self.builtin_types).neverType;
                } else {
                    let type_fun = self
                        .global_scope
                        .as_ref()
                        .unwrap()
                        .lookup_type(&alloc::string::String::from(guard_type));
                    if let Some(type_fun) = type_fun {
                        if type_fun.type_params().is_empty()
                            && type_fun.type_pack_params().is_empty()
                        {
                            let ty = follow_type_id(type_fun.r#type());

                            // We're only interested in the root type of any extern type.
                            let etv = get_type_id::<ExternType>(ty);
                            if !etv.is_null()
                                && ((*etv).parent == Some((*self.builtin_types).externType)
                                    || has_tag(ty, "typeofRoot"))
                            {
                                discriminant_ty = ty;
                            }
                        }
                    }
                }

                let proposition = self
                    .refinement_arena
                    .proposition_refinement_key_type_id(key, discriminant_ty);
                if op == AstExprBinary::CompareEq {
                    return (left_type, right_type, proposition);
                } else if op == AstExprBinary::CompareNe {
                    let negated = self.refinement_arena.negation_refinement_id(proposition);
                    return (left_type, right_type, negated);
                } else {
                    (*self.ice)
                        .ice_string("matchTypeGuard should only return a Some under `==` or `~=`!");
                    return (left_type, right_type, core::ptr::null_mut());
                }
            } else if op == AstExprBinary::CompareEq || op == AstExprBinary::CompareNe {
                // We are checking a binary expression of the form a op b
                // Just because a op b is expected to return a bool, doesn't mean a, b are expected to be bools too
                let left_type = self
                    .check_scope_ptr_ast_expr_optional_type_id_bool(scope, left, None, true)
                    .ty;
                let right_type = self
                    .check_scope_ptr_ast_expr_optional_type_id_bool(scope, right, None, true)
                    .ty;

                let left_key = (*self.dfg).get_refinement_key(left as *const AstExpr);
                let right_key = (*self.dfg).get_refinement_key(right as *const AstExpr);
                let mut left_refinement = self
                    .refinement_arena
                    .proposition_refinement_key_type_id(left_key, right_type);
                let mut right_refinement = self
                    .refinement_arena
                    .proposition_refinement_key_type_id(right_key, left_type);

                if op == AstExprBinary::CompareNe {
                    left_refinement = self
                        .refinement_arena
                        .negation_refinement_id(left_refinement);
                    right_refinement = self
                        .refinement_arena
                        .negation_refinement_id(right_refinement);
                }

                let equiv = self
                    .refinement_arena
                    .equivalence_refinement_id_refinement_id(left_refinement, right_refinement);
                return (left_type, right_type, equiv);
            } else {
                let left_type = self.check_scope_ptr_ast_expr(scope, left).ty;
                let right_type = self.check_scope_ptr_ast_expr(scope, right).ty;
                return (left_type, right_type, core::ptr::null_mut());
            }
        }
    }
}
