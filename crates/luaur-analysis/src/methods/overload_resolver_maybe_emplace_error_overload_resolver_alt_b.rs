//! Source: `Analysis/src/OverloadResolver.cpp:660-696` (hand-ported)
use crate::enums::context_error::Context;
use crate::enums::subtyping_variance::SubtypingVariance;
use crate::enums::value::Value;
use crate::functions::should_suppress_errors_type_utils::should_suppress_errors;
use crate::records::normalization_too_complex::NormalizationTooComplex;
use crate::records::overload_resolver::OverloadResolver;
use crate::records::subtyping_reasoning::SubtypingReasoning;
use crate::records::type_error::TypeError;
use crate::records::type_mismatch::TypeMismatch;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::module_name_type_fwd::ModuleName;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl OverloadResolver {
    pub fn maybe_emplace_error_error_vec_location_module_name_subtyping_reasoning_optional_type_id_optional_type_id(
        &self,
        errors: *mut ErrorVec,
        arg_location: Location,
        module_name: &ModuleName,
        reason: *const SubtypingReasoning,
        wanted_type: Option<TypeId>,
        given_type: Option<TypeId>,
    ) {
        if let (Some(wanted_type), Some(given_type)) = (wanted_type, given_type) {
            let suppression = should_suppress_errors(self.normalizer, wanted_type)
                .or_else(&should_suppress_errors(self.normalizer, given_type));

            // C++ switch on the suppression policy. The NormalizationFailed case
            // emplaces NormalizationTooComplex and then *falls through* to the
            // DoNotSuppress case, which emits the type mismatch.
            let value = suppression.error_suppression_value();
            if value == Value::Suppress {
                return;
            }

            if value == Value::NormalizationFailed {
                unsafe {
                    (*errors).push(TypeError::type_error_location_module_name_type_error_data(
                        arg_location,
                        module_name.clone(),
                        TypeErrorData::NormalizationTooComplex(NormalizationTooComplex {
                            _unused: None,
                        }),
                    ));
                }
                // intentionally fallthrough here since we couldn't prove this was error-suppressing
            }

            // DoNotSuppress (and fallthrough from NormalizationFailed):
            // TODO extract location from the SubtypingResult path and argExprs
            let reason = unsafe { &*reason };
            match reason.variance {
                SubtypingVariance::Covariant | SubtypingVariance::Contravariant => unsafe {
                    (*errors).push(TypeError::type_error_location_module_name_type_error_data(
                        arg_location,
                        module_name.clone(),
                        TypeErrorData::TypeMismatch(TypeMismatch::from_wanted_given_context(
                            wanted_type,
                            given_type,
                            Context::CovariantContext,
                        )),
                    ));
                },
                SubtypingVariance::Invariant => unsafe {
                    (*errors).push(TypeError::type_error_location_module_name_type_error_data(
                        arg_location,
                        module_name.clone(),
                        TypeErrorData::TypeMismatch(TypeMismatch::from_wanted_given_context(
                            wanted_type,
                            given_type,
                            Context::InvariantContext,
                        )),
                    ));
                },
                _ => {
                    LUAU_ASSERT!(false);
                }
            }
        }
    }
}
