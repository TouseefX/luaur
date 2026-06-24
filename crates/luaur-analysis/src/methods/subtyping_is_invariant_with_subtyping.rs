use crate::enums::subtyping_suppression_policy::SubtypingSuppressionPolicy;
use crate::enums::subtyping_variance::SubtypingVariance;
use crate::functions::assert_reasoning_valid_subtyping::assert_reasoning_valid;
use crate::functions::merge_reasonings::k_empty_reasoning;
use crate::methods::subtyping_is_contravariant_with_subtyping::IntoCovOperand;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_reasoning::SubtypingReasoning;
use crate::records::subtyping_result::SubtypingResult;
use crate::type_aliases::path::Path;
use crate::type_aliases::subtyping_reasonings::SubtypingReasonings;

impl Subtyping {
    pub fn is_invariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope<SubTy, SuperTy>(
        &mut self,
        env: &mut SubtypingEnvironment,
        sub_ty: SubTy,
        super_ty: SuperTy,
        scope: *mut Scope,
    ) -> SubtypingResult
    where
        SubTy: IntoCovOperand,
        SuperTy: IntoCovOperand,
    {
        // C++: isCovariantWith(env, subTy, superTy, scope).
        let mut result = self.covariant_dispatch(
            env,
            sub_ty.into_cov_operand(),
            super_ty.into_cov_operand(),
            scope,
        );
        let contra = self
            .is_contravariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope(
                env, sub_ty, super_ty, scope,
            );
        result.and_also(contra, SubtypingSuppressionPolicy::Any);

        if result.reasoning.empty() {
            result.reasoning.insert(SubtypingReasoning {
                sub_path: Path::default(),
                super_path: Path::default(),
                variance: SubtypingVariance::Invariant,
                is_property_modifier_violation: false,
            });
        } else {
            let mut updated = SubtypingReasonings::new(k_empty_reasoning());
            for r in result.reasoning.iter() {
                let mut r = r.clone();
                r.variance = SubtypingVariance::Invariant;
                updated.insert(r);
            }
            result.reasoning = updated;
        }

        // `assertReasoningValid` is a debug-only no-op; pass `sub_ty` for both args to
        // satisfy the single `TID` parameter (see the contravariant port for details).
        assert_reasoning_valid(sub_ty, sub_ty, &result, self.builtin_types, self.arena);

        result
    }
}
