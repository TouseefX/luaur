use crate::enums::subtyping_variance::SubtypingVariance;
use crate::functions::assert_reasoning_valid_subtyping::assert_reasoning_valid;
use crate::functions::merge_reasonings::k_empty_reasoning;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_reasoning::SubtypingReasoning;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::table_indexer::TableIndexer;
use crate::type_aliases::path::Path;
use crate::type_aliases::subtyping_reasonings::SubtypingReasonings;
use crate::type_aliases::type_id::TypeId;

/// The C++ `isContravariantWith`/`isInvariantWith` are
/// `template<typename SubTy, typename SuperTy>` and resolve the inner
/// `isCovariantWith` against the C++ overload set per instantiation. The live
/// instantiations are `TypeId/TypeId` (most calls), the dead `TryPair`
/// forwarders (`const T*/const T*`), and `TableIndexer/TableIndexer`
/// (`LuauReadOnlyIndexers`-off indexer path). We model that overload resolution
/// with a runtime tag produced by `IntoCovOperand` and dispatch to the matching
/// `isCovariantWith` overload.
pub(crate) enum CovOperand {
    Type(TypeId),
    Indexer(TableIndexer),
}

pub(crate) trait IntoCovOperand: Copy {
    fn into_cov_operand(self) -> CovOperand;
}

impl<T> IntoCovOperand for *const T {
    #[inline]
    fn into_cov_operand(self) -> CovOperand {
        CovOperand::Type(self as TypeId)
    }
}

impl IntoCovOperand for TableIndexer {
    #[inline]
    fn into_cov_operand(self) -> CovOperand {
        CovOperand::Indexer(self)
    }
}

impl Subtyping {
    /// Dispatch `isCovariantWith(env, sub, super, scope)` to the concrete overload
    /// that matches the runtime operand tags (mirrors C++ template overload
    /// resolution for the generic variance helpers).
    pub(crate) fn covariant_dispatch(
        &mut self,
        env: &mut SubtypingEnvironment,
        sub: CovOperand,
        sup: CovOperand,
        scope: *mut Scope,
    ) -> SubtypingResult {
        match (sub, sup) {
            (CovOperand::Type(a), CovOperand::Type(b)) => self
                .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                    env, a, b, scope,
                ),
            (CovOperand::Indexer(a), CovOperand::Indexer(b)) => self
                .is_covariant_with_subtyping_environment_table_indexer_table_indexer_not_null_scope(
                    env, &a, &b, scope,
                ),
            // Mixed operand kinds never occur in any real instantiation.
            _ => unreachable!("isCovariantWith dispatch with mismatched operand kinds"),
        }
    }

    pub fn is_contravariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope<
        SubTy,
        SuperTy,
    >(
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
        // C++: isCovariantWith(env, superTy, subTy, scope) — note the swap.
        let mut result = self.covariant_dispatch(
            env,
            super_ty.into_cov_operand(),
            sub_ty.into_cov_operand(),
            scope,
        );

        if result.reasoning.empty() {
            result.reasoning.insert(SubtypingReasoning {
                sub_path: Path::default(),
                super_path: Path::default(),
                variance: SubtypingVariance::Contravariant,
                is_property_modifier_violation: false,
            });
        } else {
            // If we don't swap the paths here, we will end up producing an invalid
            // path whenever we involve contravariance. We'll end up appending path
            // components that should belong to the supertype to the subtype, and vice
            // versa.
            let mut updated = SubtypingReasonings::new(k_empty_reasoning());
            for r in result.reasoning.iter() {
                let mut r = r.clone();
                core::mem::swap(&mut r.sub_path, &mut r.super_path);

                // Also swap covariant/contravariant, since those are also the other
                // way around.
                if r.variance == SubtypingVariance::Covariant {
                    r.variance = SubtypingVariance::Contravariant;
                } else if r.variance == SubtypingVariance::Contravariant {
                    r.variance = SubtypingVariance::Covariant;
                }
                updated.insert(r);
            }
            result.reasoning = updated;
        }

        // `assertReasoningValid(subTy, superTy, ...)` is a debug-only no-op (its body
        // is gated behind DebugLuauSubtypingCheckPathValidity and elided). The Rust
        // helper takes a single `TID`; `SubTy`/`SuperTy` are distinct type params here
        // (always the same kind in practice), so we pass `sub_ty` for both to satisfy
        // the unified type parameter without changing behaviour.
        assert_reasoning_valid(sub_ty, sub_ty, &result, self.builtin_types, self.arena);

        result
    }
}
