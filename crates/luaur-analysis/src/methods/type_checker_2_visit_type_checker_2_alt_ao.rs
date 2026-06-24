use crate::enums::normalization_result::NormalizationResult;
use crate::enums::type_context::TypeContext;
use crate::enums::value_context::ValueContext;
use crate::functions::find_metatable_entry::find_metatable_entry;
use crate::functions::first::first;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::has_length::has_length;
use crate::functions::is_optional::is_optional;
use crate::records::function_type::FunctionType;
use crate::records::generic_error::GenericError;
use crate::records::in_conditional_context::InConditionalContext;
use crate::records::normalization_too_complex::NormalizationTooComplex;
use crate::records::not_a_table::NotATable;
use crate::records::optional_value_access::OptionalValueAccess;
use crate::records::type_checker_2::TypeChecker2;
use crate::type_aliases::type_error_data::IntoTypeErrorData;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_unary::{AstExprUnary, AstExprUnaryOp};
use luaur_common::records::dense_hash_set::DenseHashSet;

impl TypeChecker2 {
    pub fn visit_ast_expr_unary(&mut self, expr: *mut AstExprUnary) {
        let mut in_context: Option<InConditionalContext> = None;
        unsafe {
            if (*expr).op != AstExprUnaryOp::Not {
                in_context = Some(InConditionalContext::new(
                    &mut self.type_context as *mut TypeContext,
                    TypeContext::Default,
                ));
            }
            let _ = &in_context;

            self.visit_ast_expr_value_context((*expr).expr, ValueContext::RValue);

            let operand_type = self.lookup_type((*expr).expr);
            let result_type = self.lookup_type(expr as *mut AstExpr);

            if self
                .is_error_suppressing_location_type_id((*(*expr).expr).base.location, operand_type)
            {
                return;
            }

            let k_unary_op_metamethods = [
                (AstExprUnaryOp::Minus, "__unm"),
                (AstExprUnaryOp::Len, "__len"),
            ];

            for (op, metamethod) in &k_unary_op_metamethods {
                if *op == (*expr).op {
                    let mm = find_metatable_entry(
                        self.builtin_types,
                        &mut (*self.module).errors,
                        operand_type,
                        metamethod,
                        (*expr).base.base.location,
                    );

                    if let Some(mm_ty) = mm {
                        let ftv = get_type_id::<FunctionType>(follow_type_id(mm_ty));
                        if !ftv.is_null() {
                            if let Some(ret) = first((*ftv).ret_types, false) {
                                if (*expr).op == AstExprUnaryOp::Len {
                                    self.test_is_subtype_type_id_type_id_location(
                                        follow_type_id(ret),
                                        (*self.builtin_types).numberType,
                                        (*expr).base.base.location,
                                    );
                                }
                            } else {
                                self.report_error_type_error_data_location(
                                    TypeErrorData::GenericError(GenericError::new(alloc::format!(
                                        "Metamethod '{}' must return a value",
                                        metamethod
                                    ))),
                                    &(*expr).base.base.location,
                                );
                            }

                            if first((*ftv).arg_types, false).is_none() {
                                self.report_error_type_error_data_location(
                                    TypeErrorData::GenericError(GenericError::new(
                                        "__unm metamethod must accept one argument".to_string(),
                                    )),
                                    &(*expr).base.base.location,
                                );
                                return;
                            }

                            let expected_args = (*self.module)
                                .internal_types
                                .add_type_pack_initializer_list_type_id(&[operand_type]);
                            let expected_ret = (*self.module)
                                .internal_types
                                .add_type_pack_initializer_list_type_id(&[result_type]);
                            let expected_function = (*self.module).internal_types.add_type(
                                FunctionType::function_type_new(
                                    expected_args,
                                    expected_ret,
                                    None,
                                    false,
                                ),
                            );

                            if !self.test_is_subtype_type_id_type_id_location(
                                mm_ty,
                                expected_function,
                                (*expr).base.base.location,
                            ) {
                                return;
                            }
                        }
                        return;
                    }
                    break;
                }
            }

            match (*expr).op {
                AstExprUnaryOp::Len => {
                    let mut seen: DenseHashSet<TypeId> = DenseHashSet::new(core::ptr::null());
                    let mut recursion_count = 0;
                    let nty = self.normalizer.normalize(operand_type);

                    if nty.should_suppress_errors() {
                        return;
                    }

                    match self.normalizer.is_inhabited_normalized_type(&nty) {
                        NormalizationResult::True => {}
                        NormalizationResult::False => return,
                        NormalizationResult::HitLimits => {
                            self.report_error_type_error_data_location(
                                NormalizationTooComplex::default().into_type_error_data(),
                                &(*expr).base.base.location,
                            );
                            return;
                        }
                    }

                    if !has_length(operand_type, &mut seen, &mut recursion_count) {
                        if is_optional(operand_type) {
                            self.report_error_type_error_data_location(
                                OptionalValueAccess {
                                    optional: operand_type,
                                }
                                .into_type_error_data(),
                                &(*expr).base.base.location,
                            );
                        } else {
                            self.report_error_type_error_data_location(
                                NotATable { ty: operand_type }.into_type_error_data(),
                                &(*expr).base.base.location,
                            );
                        }
                    }
                }
                AstExprUnaryOp::Minus => {
                    self.test_is_subtype_type_id_type_id_location(
                        operand_type,
                        (*self.builtin_types).numberType,
                        (*expr).base.base.location,
                    );
                }
                AstExprUnaryOp::Not => {}
            }
        }
    }
}
