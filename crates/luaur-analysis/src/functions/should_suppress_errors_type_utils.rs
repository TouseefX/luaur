use crate::enums::value::Value;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::error_suppression::ErrorSuppression;
use crate::records::normalizer::Normalizer;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::type_aliases::type_id::TypeId;

/// Rust translation of Luau.Analysis::Analysis::TypeUtils.cpp:should_suppress_errors
pub fn should_suppress_errors(normalizer: *mut Normalizer, ty: TypeId) -> ErrorSuppression {
    let ty = unsafe { follow_type_id(ty) };

    let tfit = unsafe { get_type_id::<TypeFunctionInstanceType>(ty) };
    if !tfit.is_null() {
        let tfit = unsafe { &*tfit };
        for &arg_ty in tfit.type_arguments.iter() {
            let Some(norm_type) = (unsafe { (*normalizer).try_normalize(arg_ty) }) else {
                return ErrorSuppression::from_value(Value::NormalizationFailed);
            };

            if norm_type.should_suppress_errors() {
                return ErrorSuppression::from_value(Value::Suppress);
            }
        }

        return ErrorSuppression::from_value(Value::DoNotSuppress);
    }

    let Some(norm_type) = (unsafe { (*normalizer).try_normalize(ty) }) else {
        return ErrorSuppression::from_value(Value::NormalizationFailed);
    };

    if norm_type.should_suppress_errors() {
        ErrorSuppression::from_value(Value::Suppress)
    } else {
        ErrorSuppression::from_value(Value::DoNotSuppress)
    }
}
