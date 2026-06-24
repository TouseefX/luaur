use crate::enums::normalization_result::NormalizationResult;
use crate::enums::value_context::ValueContext;
use crate::functions::should_suppress_errors_type_utils::should_suppress_errors;
use crate::records::error_suppression::ErrorSuppression;
use crate::records::normalization_too_complex::NormalizationTooComplex;
use crate::records::type_checker_2::TypeChecker2;
use crate::records::types_are_unrelated::TypesAreUnrelated;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr_type_assertion::AstExprTypeAssertion;

impl TypeChecker2 {
    pub fn visit_ast_expr_type_assertion(&mut self, expr: *mut AstExprTypeAssertion) {
        unsafe {
            self.visit_ast_expr_value_context((*expr).expr, ValueContext::RValue);
            self.visit_ast_type((*expr).annotation);

            let annotation_type: TypeId = self.lookup_annotation((*expr).annotation);
            let computed_type: TypeId = self.lookup_type((*expr).expr);

            let suppression: ErrorSuppression =
                should_suppress_errors(&mut self.normalizer as *mut _, computed_type).or_else(
                    &should_suppress_errors(&mut self.normalizer as *mut _, annotation_type),
                );

            match suppression.error_suppression_value() {
                crate::enums::value::Value::Suppress => return,
                crate::enums::value::Value::NormalizationFailed => {
                    self.report_error_type_error_data_location(
                        crate::type_aliases::type_error_data::TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
                        &(*expr).base.base.location,
                    );
                    return;
                }
                crate::enums::value::Value::DoNotSuppress => {}
            }

            match self.normalizer.is_inhabited_type_id(computed_type) {
                NormalizationResult::True => {}
                NormalizationResult::False => return,
                NormalizationResult::HitLimits => {
                    self.report_error_type_error_data_location(
                        crate::type_aliases::type_error_data::TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
                        &(*expr).base.base.location,
                    );
                    return;
                }
            }

            match self
                .normalizer
                .is_intersection_inhabited_type_id_type_id(computed_type, annotation_type)
            {
                NormalizationResult::True => return,
                NormalizationResult::False => {
                    self.report_error_type_error_data_location(
                        crate::type_aliases::type_error_data::TypeErrorData::TypesAreUnrelated(
                            TypesAreUnrelated {
                                left: computed_type,
                                right: annotation_type,
                            },
                        ),
                        &(*expr).base.base.location,
                    );
                }
                NormalizationResult::HitLimits => {
                    self.report_error_type_error_data_location(
                        crate::type_aliases::type_error_data::TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
                        &(*expr).base.base.location,
                    );
                }
            }
        }
    }
}
