use crate::functions::should_suppress_errors_type_utils_alt_b::should_suppress_errors_not_null_normalizer_type_pack_id;
use crate::records::error_suppression::ErrorSuppression;
use crate::records::normalization_too_complex::NormalizationTooComplex;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::type_checker_2::TypeChecker2;
use crate::records::type_pack_mismatch::TypePackMismatch;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::location::Location;

impl TypeChecker2 {
    pub fn explain_error_type_pack_id_type_pack_id_location_subtyping_result(
        &mut self,
        sub_tp: TypePackId,
        super_tp: TypePackId,
        location: Location,
        result: &SubtypingResult,
    ) {
        if result.is_error_suppressing {
            return;
        }

        let suppression =
            should_suppress_errors_not_null_normalizer_type_pack_id(&mut self.normalizer, sub_tp)
                .or_else(&should_suppress_errors_not_null_normalizer_type_pack_id(
                    &mut self.normalizer,
                    super_tp,
                ));

        match suppression.error_suppression_value() {
            crate::enums::value::Value::Suppress => return,
            crate::enums::value::Value::NormalizationFailed => {
                self.report_error_type_error(crate::records::type_error::TypeError::type_error_location_type_error_data(
                    location,
                    crate::type_aliases::type_error_data::TypeErrorData::NormalizationTooComplex(NormalizationTooComplex { _unused: None })
                ));
            }
            _ => {}
        }

        let mut reasonings = self
            .explain_reasonings_type_pack_id_type_pack_id_location_subtyping_result(
                sub_tp, super_tp, location, result,
            );

        if !reasonings.suppressed {
            self.report_error_type_error(
                crate::records::type_error::TypeError::type_error_location_type_error_data(
                    location,
                    crate::type_aliases::type_error_data::TypeErrorData::TypePackMismatch(
                        TypePackMismatch {
                            wanted_tp: super_tp,
                            given_tp: sub_tp,
                            reason: reasonings.to_string(),
                        },
                    ),
                ),
            );
        }
    }
}
