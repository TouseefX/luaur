use crate::functions::should_suppress_errors_type_utils_alt_b::should_suppress_errors_not_null_normalizer_type_pack_id;
use crate::records::error_suppression::ErrorSuppression;
use crate::records::normalizer::Normalizer;
use crate::type_aliases::type_pack_id::TypePackId;

pub fn should_suppress_errors(
    normalizer: *mut Normalizer,
    tp1: TypePackId,
    tp2: TypePackId,
) -> ErrorSuppression {
    let res = should_suppress_errors_not_null_normalizer_type_pack_id(normalizer, tp1);
    if res.error_suppression_value() == crate::enums::value::Value::DoNotSuppress {
        should_suppress_errors_not_null_normalizer_type_pack_id(normalizer, tp2)
    } else {
        res
    }
}
