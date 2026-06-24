use crate::functions::try_get_l_value::try_get_l_value;
use crate::functions::try_get_type_guard_predicate::try_get_type_guard_predicate;
use crate::records::and_predicate::AndPredicate;
use crate::records::eq_predicate::EqPredicate;
use crate::records::not_predicate::NotPredicate;
use crate::records::or_predicate::OrPredicate;
use crate::records::type_checker::TypeChecker;
use crate::records::with_predicate::WithPredicate;
use crate::type_aliases::predicate::Predicate;
use crate::type_aliases::predicate_vec::PredicateVec;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr_binary::{AstExprBinary, AstExprBinary_Op};

impl TypeChecker {
    pub fn check_expr_scope_ptr_ast_expr_binary_optional_type_id(
        &mut self,
        scope: &ScopePtr,
        expr: &AstExprBinary,
        expected_type: Option<TypeId>,
    ) -> WithPredicate<TypeId> {
        if expr.op == AstExprBinary_Op::And {
            let lhs = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                scope,
                unsafe { &*expr.left },
                expected_type,
                false,
            );
            let lhs_ty = lhs.r#type;
            let lhs_predicates = lhs.predicates;

            let inner_scope = self.child_scope(scope, &expr.base.base.location);
            self.resolve_predicate_vec_scope_ptr_bool(&lhs_predicates, &inner_scope, true);

            let rhs = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                &inner_scope,
                unsafe { &*expr.right },
                expected_type,
                false,
            );
            let rhs_ty = rhs.r#type;
            let rhs_predicates = rhs.predicates;

            let result_ty =
                self.check_binary_operation(scope, expr, lhs_ty, rhs_ty, &PredicateVec::new());
            WithPredicate::with_predicate_t_predicate_vec(
                result_ty,
                PredicateVec::from(alloc::vec![Predicate::And(AndPredicate {
                    lhs: lhs_predicates,
                    rhs: rhs_predicates,
                })]),
            )
        } else if expr.op == AstExprBinary_Op::Or {
            let lhs = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                scope,
                unsafe { &*expr.left },
                expected_type,
                false,
            );
            let lhs_ty = lhs.r#type;
            let lhs_predicates = lhs.predicates;

            let inner_scope = self.child_scope(scope, &expr.base.base.location);
            self.resolve_predicate_vec_scope_ptr_bool(&lhs_predicates, &inner_scope, false);

            let rhs = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                &inner_scope,
                unsafe { &*expr.right },
                expected_type,
                false,
            );
            let rhs_ty = rhs.r#type;
            let rhs_predicates = rhs.predicates;

            // Because of C++, I'm not sure if lhsPredicates was not moved out by the time we call checkBinaryOperation.
            let result = self.check_binary_operation(scope, expr, lhs_ty, rhs_ty, &lhs_predicates);
            WithPredicate::with_predicate_t_predicate_vec(
                result,
                PredicateVec::from(alloc::vec![Predicate::Or(OrPredicate {
                    lhs: lhs_predicates,
                    rhs: rhs_predicates,
                })]),
            )
        } else if expr.op == AstExprBinary_Op::CompareEq || expr.op == AstExprBinary_Op::CompareNe {
            // For these, passing expectedType is worse than simply forcing them, because their implementation
            // may inadvertently check if expectedTypes exist first and use it, instead of forceSingleton first.
            let lhs = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                scope,
                unsafe { &*expr.left },
                None,
                true,
            );
            let rhs = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                scope,
                unsafe { &*expr.right },
                None,
                true,
            );
            let lhs_ty = lhs.r#type;
            let rhs_ty = rhs.r#type;

            if let Some(predicate) = try_get_type_guard_predicate(expr) {
                return WithPredicate::with_predicate_t_predicate_vec(
                    self.boolean_type,
                    PredicateVec::from(alloc::vec![predicate]),
                );
            }

            let mut predicates: PredicateVec = PredicateVec::new();

            if let Some(lvalue) = try_get_l_value(unsafe { &*expr.left }) {
                predicates.push(Predicate::Eq(EqPredicate {
                    lvalue,
                    ty: rhs_ty,
                    location: expr.base.base.location,
                }));
            }

            if let Some(lvalue) = try_get_l_value(unsafe { &*expr.right }) {
                predicates.push(Predicate::Eq(EqPredicate {
                    lvalue,
                    ty: lhs_ty,
                    location: expr.base.base.location,
                }));
            }

            if !predicates.is_empty() && expr.op == AstExprBinary_Op::CompareNe {
                predicates =
                    PredicateVec::from(alloc::vec![Predicate::Not(NotPredicate { predicates })]);
            }

            let result_ty =
                self.check_binary_operation(scope, expr, lhs_ty, rhs_ty, &PredicateVec::new());
            WithPredicate::with_predicate_t_predicate_vec(result_ty, predicates)
        } else {
            // Expected typeArguments are not useful for other binary operators.
            let lhs = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                scope,
                unsafe { &*expr.left },
                None,
                false,
            );
            let rhs = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                scope,
                unsafe { &*expr.right },
                None,
                false,
            );
            let lhs_ty = lhs.r#type;
            let rhs_ty = rhs.r#type;
            let lhs_predicates = lhs.predicates;

            // Intentionally discarding predicates with other operators.
            let result_ty =
                self.check_binary_operation(scope, expr, lhs_ty, rhs_ty, &lhs_predicates);
            WithPredicate::with_predicate_t(result_ty)
        }
    }
}
