use crate::enums::value::Value;
use crate::functions::should_suppress_errors_type_utils_alt_b::should_suppress_errors_not_null_normalizer_type_pack_id;
use crate::records::type_checker_2::TypeChecker2;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::location::Location;

impl TypeChecker2 {
    pub fn is_error_suppressing_location_type_pack_id(
        &mut self,
        loc: Location,
        tp: TypePackId,
    ) -> bool {
        match should_suppress_errors_not_null_normalizer_type_pack_id(
            &mut self.normalizer as *mut _,
            tp,
        )
        .error_suppression_value()
        {
            Value::DoNotSuppress => false,
            Value::Suppress => true,
            Value::NormalizationFailed => {
                self.report_error_type_error_data_location(
                    crate::type_aliases::type_error_data::TypeErrorData::NormalizationTooComplex(
                        crate::records::normalization_too_complex::NormalizationTooComplex::default(
                        ),
                    ),
                    &loc,
                );
                false
            }
        }
    }
}
