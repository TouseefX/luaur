use crate::enums::value::Value;
use crate::functions::should_suppress_errors_type_utils_alt_b::should_suppress_errors_not_null_normalizer_type_pack_id;
use crate::records::error_suppression::ErrorSuppression;
use crate::records::normalization_too_complex::NormalizationTooComplex;
use crate::records::overload_resolver::OverloadResolver;
use crate::records::subtyping_reasoning::SubtypingReasoning;
use crate::records::type_pack_mismatch::TypePackMismatch;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::module_name_type_fwd::ModuleName;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::location::Location;

impl OverloadResolver {
    pub fn maybe_emplace_error_error_vec_location_module_name_subtyping_reasoning_optional_type_pack_id_optional_type_pack_id(
        &self,
        errors: *mut ErrorVec,
        arg_location: Location,
        module_name: &ModuleName,
        reason: *const SubtypingReasoning,
        wanted_tp: Option<TypePackId>,
        given_tp: Option<TypePackId>,
    ) {
        if wanted_tp.is_none() || given_tp.is_none() {
            return;
        }

        let wanted_tp = wanted_tp.unwrap();
        let given_tp = given_tp.unwrap();

        let suppression =
            should_suppress_errors_not_null_normalizer_type_pack_id(self.normalizer, wanted_tp)
                .or_else(&should_suppress_errors_not_null_normalizer_type_pack_id(
                    self.normalizer,
                    given_tp,
                ));

        match suppression.error_suppression_value() {
            Value::Suppress => {}
            Value::NormalizationFailed => unsafe {
                (*errors).push(crate::records::type_error::TypeError::type_error_location_type_error_data(
                        arg_location,
                        crate::type_aliases::type_error_data::TypeErrorData::NormalizationTooComplex(NormalizationTooComplex { _unused: None }),
                    ));
            },
            _ => unsafe {
                (*errors).push(
                    crate::records::type_error::TypeError::type_error_location_type_error_data(
                        arg_location,
                        crate::type_aliases::type_error_data::TypeErrorData::TypePackMismatch(
                            TypePackMismatch {
                                wanted_tp,
                                given_tp,
                                reason: String::new(),
                            },
                        ),
                    ),
                );
            },
        }
    }
}
