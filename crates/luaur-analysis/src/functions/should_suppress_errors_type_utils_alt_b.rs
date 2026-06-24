use crate::enums::value::Value;
use crate::functions::finite::finite;
use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::functions::should_suppress_errors_type_utils::should_suppress_errors as should_suppress_errors_not_null_normalizer_type_id;
use crate::records::any_type::AnyType;
use crate::records::error_suppression::ErrorSuppression;
use crate::records::normalizer::Normalizer;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::type_pack_id::TypePackId;

/// Rust translation of Luau.Analysis::Analysis::TypeUtils.cpp:should_suppress_errors (TypePackId overload)
pub fn should_suppress_errors_not_null_normalizer_type_pack_id(
    normalizer: *mut Normalizer,
    tp: TypePackId,
) -> ErrorSuppression {
    unsafe {
        let tp = follow_type_pack_id(tp);

        let vtp = get_type_pack_id::<VariadicTypePack>(tp);
        if !vtp.is_null() {
            let vtp = &*vtp;
            let ty = follow_type_id(vtp.ty);
            let any_ty = get_type_id::<AnyType>(ty);
            if !any_ty.is_null() {
                return ErrorSuppression::from_value(Value::Suppress);
            }
        }

        let (tys, tail) = flatten_type_pack_id(tp);

        for ty in tys {
            let result = should_suppress_errors_not_null_normalizer_type_id(normalizer, ty);
            if result != ErrorSuppression::from_value(Value::DoNotSuppress) {
                return result;
            }
        }

        if let Some(tail_tp) = tail {
            if tp != tail_tp && finite(tail_tp, core::ptr::null_mut()) {
                return should_suppress_errors_not_null_normalizer_type_pack_id(
                    normalizer, tail_tp,
                );
            }
        }

        ErrorSuppression::from_value(Value::DoNotSuppress)
    }
}
