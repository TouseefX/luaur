use crate::enums::value::Value;
use crate::functions::should_suppress_errors_type_utils::should_suppress_errors;
use crate::records::error_suppression::ErrorSuppression;
use crate::records::normalization_too_complex::NormalizationTooComplex;
use crate::records::type_checker_2::TypeChecker2;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl TypeChecker2 {
    pub fn is_error_suppressing_location_type_id(&mut self, loc: Location, ty: TypeId) -> bool {
        match should_suppress_errors(&mut self.normalizer as *mut _, ty).error_suppression_value() {
            Value::DoNotSuppress => false,
            Value::Suppress => true,
            Value::NormalizationFailed => {
                self.report_error_type_error_data_location(
                    crate::type_aliases::type_error_data::TypeErrorData::NormalizationTooComplex(
                        NormalizationTooComplex::default(),
                    ),
                    &loc,
                );
                false
            }
        }
    }
}
