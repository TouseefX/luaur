use crate::records::normalized_extern_type::NormalizedExternType;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_result::SubtypingResult;

impl Subtyping {
    pub fn is_covariant_with_subtyping_environment_normalized_extern_type_normalized_extern_type_not_null_scope(
        &mut self,
        env: &mut SubtypingEnvironment,
        sub_extern_type: &NormalizedExternType,
        super_extern_type: &NormalizedExternType,
        scope: *mut Scope,
    ) -> SubtypingResult {
        for (sub_extern_type_ty, _) in &sub_extern_type.extern_types {
            let mut result = SubtypingResult::default();

            for (super_extern_type_ty, super_negations) in &super_extern_type.extern_types {
                let candidate = self
                    .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                        env,
                        *sub_extern_type_ty,
                        *super_extern_type_ty,
                        scope,
                    );
                result.or_else(candidate);
                if !result.is_subtype {
                    continue;
                }

                for negation in &super_negations.order {
                    let negated = SubtypingResult::negate(
                        &self
                            .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                                env,
                                *sub_extern_type_ty,
                                *negation,
                                scope,
                            ),
                    );
                    result.and_also(
                        negated,
                        crate::enums::subtyping_suppression_policy::SubtypingSuppressionPolicy::Any,
                    );
                    if result.is_subtype {
                        break;
                    }
                }
            }

            if !result.is_subtype {
                return result;
            }
        }

        SubtypingResult {
            is_subtype: true,
            ..Default::default()
        }
    }
}
