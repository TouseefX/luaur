use crate::records::type_checker_2::TypeChecker2;

use crate::enums::type_context::TypeContext;
use crate::enums::value_context::ValueContext;

use crate::functions::find_metatable_entry::find_metatable_entry;
use crate::functions::get_metatable_type::get_metatable_type_id_not_null_builtin_types;
use crate::functions::is_comparison_op::is_comparison_op;
use crate::functions::is_string::is_string;
use crate::functions::op_to_meta_table_entry::op_to_meta_table_entry;
use crate::functions::strip_nil::strip_nil;

use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;

use crate::records::any_type::AnyType;
use crate::records::blocked_type::BlockedType;
use crate::records::cannot_compare_unrelated_types::CannotCompareUnrelatedTypes;
use crate::records::cannot_infer_binary_operation::CannotInferBinaryOperation;
use crate::records::free_type::FreeType;
use crate::records::generic_error::GenericError;
use crate::records::generic_type::GenericType;
use crate::records::in_conditional_context::InConditionalContext;
use crate::records::never_type::NeverType;
use crate::records::normalization_too_complex::NormalizationTooComplex;
use crate::records::table_type::TableType;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::union_type::UnionType;

use crate::enums::normalization_result::NormalizationResult;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::type_error_data::IntoTypeErrorData;
use crate::type_aliases::type_error_data::TypeErrorData;

use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_binary::AstExprBinary;
use luaur_ast::records::ast_expr_binary::AstExprBinary_Op;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_compound_assign::AstStatCompoundAssign;

use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeChecker2 {
    pub fn visit_ast_expr_binary_ast_node(
        &mut self,
        expr: *mut AstExprBinary,
        override_key: *mut AstNode,
    ) {
        let mut in_context: Option<InConditionalContext> = None;

        unsafe {
            let op = (*expr).op;
            if op != AstExprBinary_Op::And
                && op != AstExprBinary_Op::Or
                && op != AstExprBinary_Op::CompareEq
                && op != AstExprBinary_Op::CompareNe
            {
                in_context = Some(InConditionalContext::new(
                    &mut self.type_context as *mut TypeContext,
                    TypeContext::Default,
                ));
            }
        }

        unsafe {
            if luaur_common::FFlag::LuauLValueCompoundAssignmentVisitLhs.get() {
                // In compound assignments, the left side is both read-from and written-to, so we have to visit it in both contexts.
                if !override_key.is_null() && (*override_key).is::<AstStatCompoundAssign>() {
                    self.visit_ast_expr_value_context((*expr).left, ValueContext::LValue);
                }
            }

            self.visit_ast_expr_value_context((*expr).left, ValueContext::RValue);
            self.visit_ast_expr_value_context((*expr).right, ValueContext::RValue);

            let scope = *self
                .stack
                .last()
                .expect("TypeChecker2 stack should not be empty");

            let is_equality = (*expr).op == AstExprBinary_Op::CompareEq
                || (*expr).op == AstExprBinary_Op::CompareNe;
            let is_comparison = is_comparison_op((*expr).op);
            let is_logical =
                (*expr).op == AstExprBinary_Op::And || (*expr).op == AstExprBinary_Op::Or;

            let mut left_type = follow_type_id(self.lookup_type((*expr).left));
            let right_type = follow_type_id(self.lookup_type((*expr).right));
            let expected_result = follow_type_id(self.lookup_type(expr as *mut AstExpr));
            if !get_type_id::<TypeFunctionInstanceType>(expected_result).is_null() {
                self.check_for_internal_type_function(expected_result, (*expr).base.base.location);
                return;
            }

            if (*expr).op == AstExprBinary_Op::Or {
                left_type = strip_nil(
                    self.builtin_types,
                    &mut (*self.module).internal_types,
                    left_type,
                );
            }

            let norm_left = self.normalizer.try_normalize(left_type);
            let norm_right = self.normalizer.try_normalize(right_type);

            let is_string_operation = norm_left
                .as_ref()
                .map_or_else(|| is_string(left_type), |norm| norm.is_subtype_of_string())
                && norm_right
                    .as_ref()
                    .map_or_else(|| is_string(right_type), |norm| norm.is_subtype_of_string());

            left_type = follow_type_id(left_type);
            if !get_type_id::<AnyType>(left_type).is_null()
                || !get_type_id::<ErrorType>(left_type).is_null()
                || !get_type_id::<NeverType>(left_type).is_null()
            {
                return;
            } else if !get_type_id::<AnyType>(right_type).is_null()
                || !get_type_id::<ErrorType>(right_type).is_null()
                || !get_type_id::<NeverType>(right_type).is_null()
            {
                return;
            } else if norm_left
                .as_ref()
                .is_some_and(|norm| norm.should_suppress_errors())
                || norm_right
                    .as_ref()
                    .is_some_and(|norm| norm.should_suppress_errors())
            {
                return;
            }

            if (!get_type_id::<BlockedType>(left_type).is_null()
                || !get_type_id::<FreeType>(left_type).is_null()
                || !get_type_id::<GenericType>(left_type).is_null())
                && !is_equality
                && !is_logical
            {
                let name = crate::functions::get_identifier_of_base_var_type_infer::get_identifier_of_base_var(
                    (*expr).left,
                );

                self.report_error_type_error_data_location(
                    TypeErrorData::CannotInferBinaryOperation(CannotInferBinaryOperation::new(
                        (*expr).op,
                        name,
                        if is_comparison {
                            crate::enums::op_kind::OpKind::Comparison
                        } else {
                            crate::enums::op_kind::OpKind::Operation
                        },
                    )),
                    &(*expr).base.base.location,
                );
                return;
            }

            let types_have_intersection = self
                .normalizer
                .is_intersection_inhabited_type_id_type_id(left_type, right_type);

            if types_have_intersection == NormalizationResult::HitLimits {
                self.report_error_type_error_data_location(
                    NormalizationTooComplex::default().into_type_error_data(),
                    &(*expr).base.base.location,
                );
                return;
            }

            if is_equality || is_comparison {
                if !crate::functions::is_ok_to_compare::is_ok_to_compare(
                    &mut self.normalizer,
                    types_have_intersection,
                    norm_left.as_deref(),
                    norm_right.as_deref(),
                ) {
                    self.report_error_type_error_data_location(
                        TypeErrorData::CannotCompareUnrelatedTypes(CannotCompareUnrelatedTypes {
                            left: left_type,
                            right: right_type,
                            op: (*expr).op,
                        }),
                        &(*expr).base.base.location,
                    );
                    return;
                }

                let either_expr_is_nil = norm_left.as_ref().is_some_and(|norm| norm.is_nil())
                    || norm_right.as_ref().is_some_and(|norm| norm.is_nil());

                if is_equality && either_expr_is_nil {
                    return;
                }
            }

            if is_logical || (is_comparison && is_string_operation) {
                return;
            }

            let metamethod = op_to_meta_table_entry((*expr).op);
            if !metamethod.is_empty() {
                let left_mt =
                    get_metatable_type_id_not_null_builtin_types(left_type, &*self.builtin_types);
                let right_mt =
                    get_metatable_type_id_not_null_builtin_types(right_type, &*self.builtin_types);
                let mut matches = left_mt == right_mt;

                if is_equality && !matches {
                    if !matches && right_mt.is_some() {
                        let utv = get_type_id::<UnionType>(left_type);
                        if !utv.is_null() {
                            for &option in &(*utv).options {
                                if get_metatable_type_id_not_null_builtin_types(
                                    follow_type_id(option),
                                    &*self.builtin_types,
                                ) == right_mt
                                {
                                    matches = true;
                                    break;
                                }
                            }
                        }
                    }

                    if !matches && left_mt.is_some() {
                        let utv = get_type_id::<UnionType>(right_type);
                        if !utv.is_null() {
                            for &option in &(*utv).options {
                                if get_metatable_type_id_not_null_builtin_types(
                                    follow_type_id(option),
                                    &*self.builtin_types,
                                ) == left_mt
                                {
                                    matches = true;
                                    break;
                                }
                            }
                        }
                    }
                }

                if get_type_id::<TableType>(left_type).is_null()
                    && get_type_id::<TableType>(right_type).is_null()
                    && (left_mt.is_none() || right_mt.is_none())
                {
                    matches = matches || types_have_intersection != NormalizationResult::False;
                }

                if !matches && is_comparison {
                    self.report_error_type_error_data_location(
                        TypeErrorData::GenericError(GenericError::new(alloc::format!(
                            "Types {} and {} cannot be compared with {} because they do not have the same metatable",
                            crate::functions::to_string_to_string_alt_c::to_string_type_id(left_type),
                            crate::functions::to_string_to_string_alt_c::to_string_type_id(right_type),
                            luaur_ast::functions::to_string_ast_alt_b::to_string((*expr).op)
                        ))),
                        &(*expr).base.base.location,
                    );
                    return;
                }

                let left_mm = find_metatable_entry(
                    self.builtin_types,
                    &mut (*self.module).errors,
                    left_type,
                    &metamethod,
                    (*expr).base.base.location,
                );
                let right_mm = if left_mm.is_none() {
                    find_metatable_entry(
                        self.builtin_types,
                        &mut (*self.module).errors,
                        right_type,
                        &metamethod,
                        (*expr).base.base.location,
                    )
                } else {
                    None
                };

                if let Some(_mm_ty) = left_mm.or(right_mm) {
                    return;
                }

                if !is_equality
                    && !(is_string_operation
                        && ((*expr).op == AstExprBinary_Op::Concat || is_comparison))
                {
                    if (left_mt.is_some() && !is_string(left_type))
                        || (right_mt.is_some() && !is_string(right_type))
                    {
                        if is_comparison {
                            self.report_error_type_error_data_location(
                                TypeErrorData::CannotCompareUnrelatedTypes(
                                    CannotCompareUnrelatedTypes {
                                        left: left_type,
                                        right: right_type,
                                        op: (*expr).op,
                                    },
                                ),
                                &(*expr).base.base.location,
                            );
                        } else {
                            self.report_error_type_error_data_location(
                                TypeErrorData::GenericError(GenericError::new(alloc::format!(
                                    "Operator {} is not applicable for '{}' and '{}' because neither type's metatable has a '{}' metamethod",
                                    luaur_ast::functions::to_string_ast_alt_b::to_string((*expr).op),
                                    crate::functions::to_string_to_string_alt_c::to_string_type_id(left_type),
                                    crate::functions::to_string_to_string_alt_c::to_string_type_id(right_type),
                                    metamethod
                                ))),
                                &(*expr).base.base.location,
                            );
                        }
                        return;
                    } else if left_mt.is_none()
                        && right_mt.is_none()
                        && (!get_type_id::<TableType>(left_type).is_null()
                            || !get_type_id::<TableType>(right_type).is_null())
                    {
                        if is_comparison {
                            self.report_error_type_error_data_location(
                                TypeErrorData::CannotCompareUnrelatedTypes(
                                    CannotCompareUnrelatedTypes {
                                        left: left_type,
                                        right: right_type,
                                        op: (*expr).op,
                                    },
                                ),
                                &(*expr).base.base.location,
                            );
                        } else {
                            self.report_error_type_error_data_location(
                                TypeErrorData::GenericError(GenericError::new(alloc::format!(
                                    "Operator {} is not applicable for '{}' and '{}' because neither type has a metatable",
                                    luaur_ast::functions::to_string_ast_alt_b::to_string((*expr).op),
                                    crate::functions::to_string_to_string_alt_c::to_string_type_id(left_type),
                                    crate::functions::to_string_to_string_alt_c::to_string_type_id(right_type)
                                ))),
                                &(*expr).base.base.location,
                            );
                        }
                        return;
                    }
                }
            }

            match (*expr).op {
                AstExprBinary_Op::Add
                | AstExprBinary_Op::Sub
                | AstExprBinary_Op::Mul
                | AstExprBinary_Op::Div
                | AstExprBinary_Op::FloorDiv
                | AstExprBinary_Op::Pow
                | AstExprBinary_Op::Mod => {
                    self.test_is_subtype_type_id_type_id_location(
                        left_type,
                        (*self.builtin_types).numberType,
                        (*(*expr).left).base.location,
                    );
                    self.test_is_subtype_type_id_type_id_location(
                        right_type,
                        (*self.builtin_types).numberType,
                        (*(*expr).right).base.location,
                    );
                }
                AstExprBinary_Op::Concat => {
                    let number_or_string = (*self.module).internal_types.add_type(UnionType {
                        options: vec![
                            (*self.builtin_types).numberType,
                            (*self.builtin_types).stringType,
                        ],
                    });
                    self.test_is_subtype_type_id_type_id_location(
                        left_type,
                        number_or_string,
                        (*(*expr).left).base.location,
                    );
                    self.test_is_subtype_type_id_type_id_location(
                        right_type,
                        number_or_string,
                        (*(*expr).right).base.location,
                    );
                }
                AstExprBinary_Op::CompareGe
                | AstExprBinary_Op::CompareGt
                | AstExprBinary_Op::CompareLe
                | AstExprBinary_Op::CompareLt => {
                    if norm_left
                        .as_ref()
                        .is_some_and(|norm| norm.should_suppress_errors())
                    {
                        return;
                    }

                    if norm_left.as_ref().is_some_and(|norm| {
                        self.normalizer.is_inhabited_normalized_type(norm)
                            == NormalizationResult::False
                    }) {
                        return;
                    }

                    if (*self.subtyping)
                        .is_subtype_type_id_type_id_not_null_scope(
                            left_type,
                            (*self.builtin_types).numberType,
                            scope,
                        )
                        .is_subtype
                    {
                        self.test_is_subtype_type_id_type_id_location(
                            right_type,
                            (*self.builtin_types).numberType,
                            (*(*expr).right).base.location,
                        );
                        return;
                    }

                    if (*self.subtyping)
                        .is_subtype_type_id_type_id_not_null_scope(
                            left_type,
                            (*self.builtin_types).stringType,
                            scope,
                        )
                        .is_subtype
                    {
                        self.test_is_subtype_type_id_type_id_location(
                            right_type,
                            (*self.builtin_types).stringType,
                            (*(*expr).right).base.location,
                        );
                        return;
                    }

                    self.report_error_type_error_data_location(
                        TypeErrorData::GenericError(GenericError::new(alloc::format!(
                            "Types '{}' and '{}' cannot be compared with relational operator {}",
                            crate::functions::to_string_to_string_alt_c::to_string_type_id(
                                left_type
                            ),
                            crate::functions::to_string_to_string_alt_c::to_string_type_id(
                                right_type
                            ),
                            luaur_ast::functions::to_string_ast_alt_b::to_string((*expr).op)
                        ))),
                        &(*expr).base.base.location,
                    );
                    return;
                }
                AstExprBinary_Op::And
                | AstExprBinary_Op::Or
                | AstExprBinary_Op::CompareEq
                | AstExprBinary_Op::CompareNe
                | AstExprBinary_Op::Op__Count => {}
            }

            let _ = in_context;
            let _ = types_have_intersection;
        }
    }
}
