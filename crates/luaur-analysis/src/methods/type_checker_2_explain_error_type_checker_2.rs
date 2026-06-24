use crate::functions::should_suppress_errors_type_utils::should_suppress_errors;
use crate::records::error_suppression::ErrorSuppression;
use crate::records::normalization_too_complex::NormalizationTooComplex;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::type_checker_2::TypeChecker2;
use crate::records::type_mismatch::TypeMismatch;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl TypeChecker2 {
    pub fn explain_error_type_id_type_id_location_subtyping_result(
        &mut self,
        sub_ty: TypeId,
        super_ty: TypeId,
        location: Location,
        result: &SubtypingResult,
    ) {
        if result.is_error_suppressing {
            return;
        }

        let suppression = should_suppress_errors(&mut self.normalizer, sub_ty)
            .or_else(&should_suppress_errors(&mut self.normalizer, super_ty));

        match suppression.error_suppression_value() {
            crate::enums::value::Value::Suppress => return,
            crate::enums::value::Value::NormalizationFailed => {
                self.report_error_type_error(
                    crate::records::type_error::TypeError::type_error_location_type_error_data(
                        location,
                        TypeErrorData::NormalizationTooComplex(NormalizationTooComplex {
                            _unused: None,
                        }),
                    ),
                );
            }
            _ => {}
        }

        let mut reasonings = self.explain_reasonings_type_id_type_id_location_subtyping_result(
            sub_ty, super_ty, location, result,
        );

        if !reasonings.suppressed {
            self.report_error_type_error(
                crate::records::type_error::TypeError::type_error_location_type_error_data(
                    location,
                    TypeErrorData::TypeMismatch(TypeMismatch {
                        wanted_type: super_ty,
                        given_type: sub_ty,
                        reason: reasonings.to_string(),
                        error: None,
                        context: crate::enums::context_error::Context::Covariant,
                    }),
                ),
            );
        }
    }
}
