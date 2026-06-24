use crate::functions::first::first;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::has_length::has_length;
use crate::functions::type_could_have_metatable::type_could_have_metatable;
use crate::records::any_type::AnyType;
use crate::records::error_type::ErrorType;
use crate::records::function_type::FunctionType;
use crate::records::generic_error::GenericError;
use crate::records::never_type::NeverType;
use crate::records::not_a_table::NotATable;
use crate::records::not_predicate::NotPredicate;
use crate::records::type_checker::TypeChecker;
use crate::records::with_predicate::WithPredicate;
use crate::type_aliases::predicate::Predicate;
use crate::type_aliases::predicate_vec::PredicateVec;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use alloc::format;
use alloc::string::ToString;
use luaur_ast::records::ast_expr_unary::{AstExprUnary, AstExprUnaryOp};
use luaur_common::records::dense_hash_set::DenseHashSet;

impl TypeChecker {
    pub fn check_expr_scope_ptr_ast_expr_unary(
        &mut self,
        scope: &ScopePtr,
        expr: &AstExprUnary,
    ) -> WithPredicate<TypeId> {
        let boolean_type = self.boolean_type;
        let number_type = self.number_type;
        let nil_type = self.nil_type;

        let result = self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
            scope,
            unsafe { &*expr.expr },
            None,
            false,
        );
        let mut operand_type = unsafe { follow_type_id(result.r#type) };

        match expr.op {
            AstExprUnaryOp::Not => WithPredicate::with_predicate_t_predicate_vec(
                boolean_type,
                PredicateVec::from(alloc::vec![Predicate::Not(NotPredicate {
                    predicates: result.predicates,
                })]),
            ),
            AstExprUnaryOp::Minus => {
                let operand_is_any = unsafe {
                    !get_type_id::<AnyType>(operand_type).is_null()
                        || !get_type_id::<ErrorType>(operand_type).is_null()
                        || !get_type_id::<NeverType>(operand_type).is_null()
                };

                if operand_is_any {
                    return WithPredicate::with_predicate_t(operand_type);
                }

                if type_could_have_metatable(operand_type) {
                    if let Some(fnt) = self.find_metatable_entry(
                        operand_type,
                        "__unm".to_string(),
                        &expr.base.base.location,
                        true,
                    ) {
                        let actual_function_type = self.instantiate(
                            scope,
                            fnt,
                            expr.base.base.location,
                            core::ptr::null(),
                        );
                        let arguments =
                            self.add_type_pack_initializer_list_type_id(&[operand_type]);
                        let ret_type_pack = self.fresh_type_pack_scope_ptr(scope.clone());
                        let mut ftv =
                            FunctionType::function_type_new(arguments, ret_type_pack, None, false);
                        ftv.level = scope.level;
                        let expected_function_type = self.add_type_tv_internal(ftv);

                        let mut state = self.mk_unifier(scope, &expr.base.base.location);
                        state.try_unify_type_id_type_id_bool_bool_literal_properties_entry(
                            actual_function_type,
                            expected_function_type,
                            true,
                            false,
                            None,
                        );
                        state.log.commit();

                        self.report_errors(&state.errors);
                        let has_errors = !state.errors.is_empty();

                        let mut ret_type = first(ret_type_pack, false).unwrap_or(nil_type);
                        if has_errors {
                            ret_type = self.error_recovery_type_type_id(ret_type);
                        }

                        return WithPredicate::with_predicate_t(ret_type);
                    }

                    self.report_error_location_type_error_data(
                        &expr.base.base.location,
                        TypeErrorData::GenericError(GenericError::new(format!(
                            "Unary operator '{}' not supported by type '{}'",
                            luaur_ast::functions::to_string_ast::to_string(expr.op),
                            crate::functions::to_string_to_string_alt_c::to_string_type_id(
                                operand_type
                            )
                        ))),
                    );
                    return WithPredicate::with_predicate_t(
                        self.error_recovery_type_scope_ptr(scope),
                    );
                }

                let errs =
                    self.try_unify(operand_type, number_type, scope, &expr.base.base.location);
                self.report_errors(&errs);
                WithPredicate::with_predicate_t(number_type)
            }
            AstExprUnaryOp::Len => {
                self.tablify(operand_type);

                operand_type =
                    self.strip_from_nil_and_report(operand_type, &expr.base.base.location);

                // # operator is guaranteed to return number
                if unsafe {
                    !get_type_id::<AnyType>(operand_type).is_null()
                        || !get_type_id::<ErrorType>(operand_type).is_null()
                        || !get_type_id::<NeverType>(operand_type).is_null()
                } {
                    return WithPredicate::with_predicate_t(number_type);
                }

                let mut seen: DenseHashSet<TypeId> = DenseHashSet::new(core::ptr::null());

                if type_could_have_metatable(operand_type) {
                    if let Some(fnt) = self.find_metatable_entry(
                        operand_type,
                        "__len".to_string(),
                        &expr.base.base.location,
                        true,
                    ) {
                        let actual_function_type = self.instantiate(
                            scope,
                            fnt,
                            expr.base.base.location,
                            core::ptr::null(),
                        );
                        let arguments =
                            self.add_type_pack_initializer_list_type_id(&[operand_type]);
                        let ret_type_pack =
                            self.add_type_pack_initializer_list_type_id(&[number_type]);
                        let mut ftv =
                            FunctionType::function_type_new(arguments, ret_type_pack, None, false);
                        ftv.level = scope.level;
                        let expected_function_type = self.add_type_tv_internal(ftv);

                        let mut state = self.mk_unifier(scope, &expr.base.base.location);
                        state.try_unify_type_id_type_id_bool_bool_literal_properties_entry(
                            actual_function_type,
                            expected_function_type,
                            true,
                            false,
                            None,
                        );
                        state.log.commit();

                        self.report_errors(&state.errors);
                    }
                }

                if !has_length(operand_type, &mut seen, &mut self.recursion_count) {
                    self.report_error_location_type_error_data(
                        &expr.base.base.location,
                        TypeErrorData::NotATable(NotATable { ty: operand_type }),
                    );
                }

                WithPredicate::with_predicate_t(number_type)
            }
        }
    }
}
