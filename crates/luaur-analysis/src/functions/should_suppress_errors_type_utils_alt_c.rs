use crate::functions::should_suppress_errors_type_utils::should_suppress_errors as should_suppress_errors_single;
use crate::records::error_suppression::ErrorSuppression;
use crate::records::normalizer::Normalizer;
use crate::type_aliases::type_id::TypeId;

pub fn should_suppress_errors(
    normalizer: *mut Normalizer,
    ty1: TypeId,
    ty2: TypeId,
) -> ErrorSuppression {
    let res = should_suppress_errors_single(normalizer, ty1);
    if res.error_suppression_value() == crate::enums::value::Value::DoNotSuppress {
        should_suppress_errors_single(normalizer, ty2)
    } else {
        res
    }
}
